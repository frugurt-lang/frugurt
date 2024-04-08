{-# LANGUAGE LambdaCase #-}

module Debugize (toDbgStrStmt) where

import Data.List (intercalate)
import Data.Maybe (isJust)
import Treeanize (FruExpr (..), FruField (..), FruMethod (..), FruStmt (..), FruWatch (..))


getSpace :: Int -> String
getSpace indent = concat $ replicate indent "|    "


toDbgStrField :: Int -> FruField -> String
toDbgStrField indent (FruField isPub isStatic name typeIdent) =
  getSpace indent
    ++ (if isPub then "pub " else "")
    ++ (if isJust isStatic then "static " else "")
    ++ name
    ++ ( case typeIdent of
          Nothing -> ""
          Just ident -> " : " ++ ident
       )
    ++ (case isStatic of Just (Just v) -> " =\n" ++ toDbgStrExpr (indent + 1) v; _ -> "\n")


toDbgWatch :: Int -> FruWatch -> String
toDbgWatch indent (FruWatch fields body) =
  getSpace indent
    ++ "Watch:\n"
    ++ getSpace (indent + 1)
    ++ "fields: "
    ++ intercalate ", " fields
    ++ "\n"
    ++ getSpace (indent + 1)
    ++ "body:\n"
    ++ toDbgStrStmt (indent + 2) body


toDbgStrMethod :: Int -> FruMethod -> String
toDbgStrMethod indent (FruMethod name args body) =
  getSpace indent
    ++ "Method:\n"
    ++ getSpace (indent + 1)
    ++ "name: "
    ++ name
    ++ "\n"
    ++ getSpace (indent + 1)
    ++ "args: "
    ++ intercalate ", " args
    ++ "\n"
    ++ getSpace (indent + 1)
    ++ "body:\n"
    ++ toDbgStrStmt (indent + 2) body


toDbgStrProperty :: Int -> String -> a -> (Int -> a -> String) -> String
toDbgStrProperty indent name value toString =
  getSpace (indent + 1)
    ++ name
    ++ ":\n"
    ++ toString (indent + 2) value


toDbgStrStr :: Int -> String -> String -- TODO: make use of this everywhere
toDbgStrStr indent str = getSpace indent ++ str ++ "\n"


toDbgStrStmt :: Int -> FruStmt -> String
toDbgStrStmt indent = \case
  StBlock stmts ->
    getSpace indent
      ++ "Block:\n"
      ++ concatMap (toDbgStrStmt (indent + 1)) stmts
  StNothing ->
    getSpace indent
      ++ "Nothing:\n"
  StExpr e ->
    getSpace indent
      ++ "Expression:\n"
      ++ toDbgStrExpr (indent + 1) e
  StLet ident e ->
    getSpace indent
      ++ "Let:\n"
      ++ toDbgStrProperty indent "ident" ident toDbgStrStr
      ++ toDbgStrProperty indent "what" e toDbgStrExpr
  StSet ident e ->
    getSpace indent
      ++ "Set:\n"
      ++ toDbgStrProperty indent "path" ident toDbgStrStr
      ++ toDbgStrProperty indent "what" e toDbgStrExpr
  StSetField target field e ->
    getSpace indent
      ++ "SetField:\n"
      ++ toDbgStrProperty indent "target" target toDbgStrExpr
      ++ toDbgStrProperty indent "field" field toDbgStrStr
      ++ toDbgStrProperty indent "what" e toDbgStrExpr
  StIf cond thenBody elseBody ->
    getSpace indent
      ++ "If:\n"
      ++ toDbgStrProperty indent "cond" cond toDbgStrExpr
      ++ toDbgStrProperty indent "then" thenBody toDbgStrStmt
      ++ toDbgStrProperty indent "else" elseBody toDbgStrStmt
  StWhile cond body ->
    getSpace indent
      ++ "While:\n"
      ++ toDbgStrProperty indent "cond" cond toDbgStrExpr
      ++ toDbgStrProperty indent "body" body toDbgStrStmt
  StReturn e ->
    getSpace indent
      ++ "Return:\n"
      ++ toDbgStrExpr (indent + 1) e
  StBreak ->
    getSpace indent
      ++ "Break\n"
  StContinue ->
    getSpace indent
      ++ "Continue\n"
  StOperator op commutative leftIdent leftType rightIdent rightType body ->
    getSpace indent
      ++ (if commutative then "Commutative " else "")
      ++ "Operator:\n"
      ++ toDbgStrProperty indent "op" op toDbgStrStr
      ++ toDbgStrProperty indent "left" (leftIdent ++ " : " ++ leftType) toDbgStrStr
      ++ toDbgStrProperty indent "right" (rightIdent ++ " : " ++ rightType) toDbgStrStr
      ++ toDbgStrProperty indent "body" body toDbgStrStmt
  StType typeType ident fields watches methods staticMethods ->
    getSpace indent
      ++ "Type:\n"
      ++ toDbgStrProperty indent "type" typeType toDbgStrStr
      ++ toDbgStrProperty indent "ident" ident toDbgStrStr
      ++ getSpace (indent + 1)
      ++ "fields:\n"
      ++ concatMap (toDbgStrField (indent + 2)) fields
      ++ getSpace (indent + 1)
      ++ "watches:\n"
      ++ concatMap (toDbgWatch (indent + 2)) watches
      ++ getSpace (indent + 1)
      ++ "methods:\n"
      ++ concatMap (toDbgStrMethod (indent + 2)) methods
      ++ getSpace (indent + 1)
      ++ "static methods:\n"
      ++ concatMap (toDbgStrMethod (indent + 2)) staticMethods


toDbgStrExpr :: Int -> FruExpr -> String
toDbgStrExpr indent = \case
  ExLiteralNah -> getSpace indent ++ "nah\n"
  ExLiteralNumber n -> getSpace indent ++ show n ++ "\n"
  ExLiteralBool b -> getSpace indent ++ show b ++ "\n"
  ExLiteralString s -> getSpace indent ++ show s ++ "\n"
  ExVariable v -> getSpace indent ++ v ++ "\n"
  ExBlock body expr ->
    getSpace indent
      ++ "Block:\n"
      ++ concatMap (toDbgStrStmt (indent + 1)) body
      ++ getSpace (indent + 1)
      ++ "expression:\n"
      ++ toDbgStrExpr (indent + 1) expr
  ExCall e es ->
    getSpace indent
      ++ "Call:\n"
      ++ toDbgStrProperty indent "what" e toDbgStrExpr
      ++ getSpace (indent + 1)
      ++ "args:\n"
      ++ concatMap (toDbgStrExpr (indent + 2)) es
  ExCurryCall e es ->
    getSpace indent
      ++ "CurryCall:\n"
      ++ toDbgStrProperty indent "what" e toDbgStrExpr
      ++ getSpace (indent + 1)
      ++ "args:\n"
      ++ concatMap (toDbgStrExpr (indent + 2)) es
  ExBinaries f r ->
    getSpace indent
      ++ "Binaries:\n"
      ++ toDbgStrExpr (indent + 1) f
      ++ concatMap (\(op, ex) -> toDbgStrStr (indent + 1) op ++ toDbgStrExpr (indent + 1) ex) r
  ExFunction args body ->
    getSpace indent
      ++ "Function:\n"
      ++ toDbgStrProperty indent "args" (intercalate ", " args) toDbgStrStr
      ++ toDbgStrProperty indent "body" body toDbgStrStmt
  ExInstantiation e es ->
    getSpace indent
      ++ "Instantiation:\n"
      ++ toDbgStrProperty indent "what" e toDbgStrExpr
      ++ getSpace (indent + 1)
      ++ "args:\n"
      ++ concatMap (toDbgStrExpr (indent + 2)) es
  ExFieldAccess e f ->
    getSpace indent
      ++ "FieldAccess:\n"
      ++ toDbgStrProperty indent "what" e toDbgStrExpr
      ++ toDbgStrProperty indent "field" f toDbgStrStr
  ExIfElse condition thenBody elseBody ->
    getSpace indent
      ++ "IfElse:\n"
      ++ toDbgStrProperty indent "condition" condition toDbgStrExpr
      ++ toDbgStrProperty indent "then" thenBody toDbgStrExpr
      ++ toDbgStrProperty indent "else" elseBody toDbgStrExpr
