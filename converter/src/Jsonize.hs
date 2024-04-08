{-# LANGUAGE LambdaCase #-}

module Jsonize (toJsonExpr, toJsonStmt, toString, JSON (..)) where

import Data.List (intercalate)
import Data.Maybe (isJust)
import Data.Scientific (toRealFloat)
import Treeanize (FruExpr (..), FruField (..), FruMethod (..), FruStmt (..), FruWatch (..))


data JSON
  = Number Double
  | Bool Bool
  | Null
  | Str String
  | Array [JSON]
  | Object [(String, JSON)]
  deriving (Show, Eq)


toJsonExpr :: FruExpr -> JSON
toJsonExpr = \case
  ExLiteralNah ->
    Object
      [ ("node", Str "literal")
      , ("value", Null)
      ]
  ExLiteralNumber i ->
    Object
      [ ("node", Str "literal")
      , ("value", Number $ toRealFloat i)
      ]
  ExLiteralBool b ->
    Object
      [ ("node", Str "literal")
      , ("value", Bool b)
      ]
  ExLiteralString s ->
    Object
      [ ("node", Str "literal")
      , ("value", Str s)
      ]
  ExVariable ident ->
    Object
      [ ("node", Str "variable")
      , ("ident", Str ident)
      ]
  ExFunction args body ->
    Object
      [ ("node", Str "function")
      , ("args", Array $ map Str args)
      , ("body", toJsonStmt body)
      ]
  ExBlock body expr ->
    Object
      [ ("node", Str "block")
      , ("body", Array $ map toJsonStmt body)
      , ("expr", toJsonExpr expr)
      ]
  ExCall what args ->
    Object
      [ ("node", Str "call")
      , ("what", toJsonExpr what)
      , ("args", Array $ map toJsonExpr args)
      ]
  ExCurryCall what args ->
    Object
      [ ("node", Str "curry")
      , ("what", toJsonExpr what)
      , ("args", Array $ map toJsonExpr args)
      ]
  ExInstantiation what args ->
    Object
      [ ("node", Str "instantiation")
      , ("what", toJsonExpr what)
      , ("args", Array $ map toJsonExpr args)
      ]
  ExFieldAccess what field ->
    Object
      [ ("node", Str "field_access")
      , ("what", toJsonExpr what)
      , ("field", Str field)
      ]
  ExBinaries first rest ->
    Object
      [ ("node", Str "binaries")
      , ("first", toJsonExpr first)
      , ("rest", Array $ map (\(op, ex) -> Array [Str op, toJsonExpr ex]) rest)
      ]
  ExIfElse condition thenBody elseBody ->
    Object
      [ ("node", Str "if_expr")
      , ("cond", toJsonExpr condition)
      , ("then", toJsonExpr thenBody)
      , ("else", toJsonExpr elseBody)
      ]


toJsonStmt :: FruStmt -> JSON
toJsonStmt stmt = case stmt of
  StBlock body ->
    Object
      [ ("node", Str "block")
      , ("body", Array $ map toJsonStmt body)
      ]
  StNothing -> Object [("node", Str "nothing")]
  StExpr expression ->
    Object
      [ ("node", Str "expression")
      , ("value", toJsonExpr expression)
      ]
  StLet ident expression ->
    Object
      [ ("node", Str "let")
      , ("ident", Str ident)
      , ("value", toJsonExpr expression)
      ]
  StSet ident expression ->
    Object
      [ ("node", Str "set")
      , ("ident", Str ident)
      , ("value", toJsonExpr expression)
      ]
  StSetField target field expression ->
    Object
      [ ("node", Str "set_field")
      , ("target", toJsonExpr target)
      , ("field", Str field)
      , ("value", toJsonExpr expression)
      ]
  StIf cond thenBody elseBody ->
    Object
      [ ("node", Str "if")
      , ("cond", toJsonExpr cond)
      , ("then", toJsonStmt thenBody)
      , ("else", toJsonStmt elseBody)
      ]
  StWhile cond body ->
    Object
      [ ("node", Str "while")
      , ("cond", toJsonExpr cond)
      , ("body", toJsonStmt body)
      ]
  StReturn expression ->
    Object
      [ ("node", Str "return")
      , ("value", toJsonExpr expression)
      ]
  StBreak -> Object [("node", Str "break")]
  StContinue -> Object [("node", Str "continue")]
  StOperator op commutative left_arg left_type right_arg right_type body ->
    Object
      [ ("node", Str "operator")
      , ("ident", Str op)
      , ("commutative", Bool commutative)
      , ("left_ident", Str left_arg)
      , ("left_type_ident", Str left_type)
      , ("right_ident", Str right_arg)
      , ("right_type_ident", Str right_type)
      , ("body", toJsonStmt body)
      ]
  StType t ident fields watches methods staticMethods ->
    Object
      [ ("node", Str "type")
      , ("type", Str t)
      , ("ident", Str ident)
      , ("fields", Array $ map toJsonField fields)
      , ("watches", Array $ map toJsonWatch watches)
      , ("methods", Array $ map toJsonMethod methods)
      , ("static_methods", Array $ map toJsonMethod staticMethods)
      ]


toJsonField :: FruField -> JSON
toJsonField (FruField isPub static ident typeIdent) =
  Object $
    [ ("is_pub", Bool isPub)
    , ("is_static", Bool $ isJust static)
    , ("ident", Str ident)
    ]
      ++ (case typeIdent of Just typeIdent_ -> [("type_ident", Str typeIdent_)]; _ -> [])
      ++ (case static of Just (Just v) -> [("value", toJsonExpr v)]; _ -> [])


toJsonWatch :: FruWatch -> JSON
toJsonWatch (FruWatch flds body) = Object [("fields", Array $ map Str flds), ("body", toJsonStmt body)]


toJsonMethod :: FruMethod -> JSON
toJsonMethod (FruMethod name args body) =
  Object
    [ ("ident", Str name)
    , ("args", Array $ map Str args)
    , ("body", toJsonStmt body)
    ]


toString :: JSON -> String
toString (Number i) = show i
toString (Bool b)
  | b = "true"
  | otherwise = "false"
toString Null = "null"
toString (Str s) = show s
toString (Array xs) =
  "["
    ++ intercalate ", " (map toString xs)
    ++ "]"
toString (Object xs) =
  "{"
    ++ intercalate
      ", "
      (map (\(k, v) -> "\"" ++ k ++ "\"" ++ ": " ++ toString v) xs)
    ++ "}"