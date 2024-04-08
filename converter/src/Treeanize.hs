{-# LANGUAGE LambdaCase #-}
{-# OPTIONS_GHC -Wno-partial-fields #-}

module Treeanize (toAst, FruExpr (..), FruStmt (..), FruWatch (..), FruField (..), FruMethod (..)) where

import Control.Monad (when)
import qualified Data.List.NonEmpty as NonEmpty
import Data.Maybe (isJust)
import Data.Scientific (Scientific)
import Data.Set (Set, singleton)
import Data.Void (Void)
import Text.Megaparsec
  ( MonadParsec (eof, token, try)
  , Parsec
  , between
  , choice
  , failure
  , many
  , oneOf
  , optional
  , sepBy
  , single
  , (<|>)
  )
import Text.Megaparsec.Error (ErrorItem (Label))
import Tokenize (FruToken (..))


data FruExpr
  = ExLiteralNah
  | ExLiteralNumber Scientific
  | ExLiteralBool Bool
  | ExLiteralString String
  | ExVariable String
  | ExFunction [String] FruStmt
  | ExBlock [FruStmt] FruExpr
  | ExCall FruExpr [FruExpr]
  | ExCurryCall FruExpr [FruExpr]
  | ExInstantiation FruExpr [FruExpr] -- type object * field values
  | ExFieldAccess FruExpr String
  | ExBinaries FruExpr [(String, FruExpr)]
  | ExIfElse FruExpr FruExpr FruExpr
  deriving (Show, Eq)


data FruStmt
  = StBlock [FruStmt] -- new scope always
  | StNothing -- same as (StBlock [])
  | StExpr FruExpr
  | StLet String FruExpr
  | StSet String FruExpr
  | StSetField FruExpr String FruExpr
  | StIf FruExpr FruStmt FruStmt -- always StBlock for 2 and 3 fields
  | StWhile FruExpr FruStmt
  | StReturn FruExpr
  | StBreak
  | StContinue
  | StOperator String Bool String String String String FruStmt -- operator ident * is commutative * left ident * left type ident * right ident * right type ident * body
  | StType
      { getTypeType :: String
      , getIdent :: String
      , getFields :: [FruField]
      , getWatches :: [FruWatch]
      , getMethods :: [FruMethod]
      , getStaticMethods :: [FruMethod]
      }
  deriving (Show, Eq)


-- helpers

makeErrSet :: String -> Set (ErrorItem FruToken)
makeErrSet = singleton . Label . NonEmpty.fromList


binaryOp :: Parsec Void [FruToken] String
binaryOp = token (\case TkOp "=" -> Nothing; TkOp x -> Just x; _ -> Nothing) (makeErrSet "bynary operator")


-- oop stuff

data FruField
  = FruField Bool (Maybe (Maybe FruExpr)) String (Maybe String) -- is pub * is static && value * ident * type
  deriving (Show, Eq)


data FruWatch
  = FruWatch [String] FruStmt
  deriving (Show, Eq)


data FruMethod
  = FruMethod String [String] FruStmt
  deriving (Show, Eq)


data TypeSection
  = FieldsSection [FruField]
  | ConstraintSection [FruWatch]
  | ImplSection [FruMethod]
  | StaticSection [FruMethod]


composeType :: FruToken -> String -> [TypeSection] -> FruStmt
composeType typeType ident = foldl applySection basicType
  where
    basicType = StType (typeTypeToStr typeType) ident [] [] [] []
    typeTypeToStr = \case
      TkStruct -> "struct"
      _ -> undefined

    applySection :: FruStmt -> TypeSection -> FruStmt
    applySection to@(StType _ _ fields watches methods staticMethods) = \case
      FieldsSection fields' -> to{getFields = fields ++ fields'}
      ConstraintSection watches' -> to{getWatches = watches ++ watches'}
      ImplSection methods' -> to{getMethods = methods ++ methods'}
      StaticSection staticMethods' -> to{getStaticMethods = staticMethods ++ staticMethods'}
    applySection _ = undefined


-- parser

type ParserStmt = Parsec Void [FruToken] FruStmt


type ParserExpr = Parsec Void [FruToken] FruExpr


type ParserExtExpr = Parsec Void [FruToken] (FruExpr -> FruExpr)


identifier :: Parsec Void [FruToken] String
identifier = token (\case TkIdent x -> Just x; _ -> Nothing) (makeErrSet "identifier")


toAst :: ParserStmt
toAst = program
  where
    program :: ParserStmt
    program = do
      stmts <- many statement <* eof
      return $ StBlock stmts

    statement :: ParserStmt
    statement =
      choice
        [ blockStmt
        , try letStmt
        , try setStmt
        , try setFieldStmt
        , try ifElseStmt
        , try ifStmt
        , try whileStmt
        , try returnStmt
        , try breakStmt
        , try continueStmt
        , try operatorStmt
        , try typeStmt
        , try exprStmt
        ]

    blockStmt :: ParserStmt
    blockStmt =
      try $
        StBlock
          <$> between
            (single TkBraceOpen)
            (single TkBraceClose)
            (many statement)

    blockExpr :: ParserExpr
    blockExpr = do
      _ <- single TkBraceOpen
      stmts <- many statement
      expr <- expression
      _ <- single TkBraceClose

      return $ ExBlock stmts expr

    functionBodyStmt :: ParserStmt
    functionBodyStmt = blockStmt <|> (blockExprTransform <$> blockExpr)
      where
        blockExprTransform (ExBlock body expr) = StBlock $ body ++ [StReturn expr]
        blockExprTransform _ = error "unreachable"

    exprStmt :: ParserStmt
    exprStmt = do
      ex <- expression
      _ <- single TkSemiColon
      return $ StExpr ex

    letStmt :: ParserStmt
    letStmt = do
      _ <- single TkLet
      name <- identifier
      _ <- single (TkOp "=")
      value <- expression
      _ <- single TkSemiColon
      return $ StLet name value

    setStmt :: ParserStmt
    setStmt = do
      ident <- identifier
      _ <- single (TkOp "=")
      value <- expression
      _ <- single TkSemiColon
      return $ StSet ident value

    setFieldStmt :: ParserStmt
    setFieldStmt = do
      target <- expression

      when ((\case ExFieldAccess _ _ -> False; _ -> True) target) $ do
        failure Nothing (makeErrSet "field access")

      let (target', field) = case target of
            ExFieldAccess t f -> (t, f)
            _ -> error "unreachable"

      _ <- single (TkOp "=")
      value <- expression
      _ <- single TkSemiColon

      return $ StSetField target' field value

    ifStmt :: ParserStmt
    ifStmt = do
      _ <- single TkIf
      cond <- expression
      thenBody <- blockStmt
      return $ StIf cond thenBody StNothing

    ifElseStmt :: ParserStmt
    ifElseStmt = do
      _ <- single TkIf
      cond <- expression
      thenBody <- blockStmt
      _ <- single TkElse
      StIf cond thenBody <$> (blockStmt <|> ifElseStmt <|> ifStmt)

    whileStmt :: ParserStmt
    whileStmt = do
      _ <- single TkWhile
      cond <- expression
      StWhile cond <$> blockStmt

    returnStmt :: ParserStmt
    returnStmt = do
      _ <- single TkReturn
      value <- expression
      _ <- single TkSemiColon
      return $ StReturn value

    breakStmt :: ParserStmt
    breakStmt = StBreak <$ single TkBreak <* single TkSemiColon

    continueStmt :: ParserStmt
    continueStmt = StContinue <$ single TkContinue <* single TkSemiColon

    operatorStmt :: ParserStmt
    operatorStmt = do
      commutative <- isJust <$> optional (single TkCommutative)

      _ <- single TkOperator
      ident <- token (\case TkOp x -> Just x; _ -> Nothing) (makeErrSet "operator")

      _ <- single TkParenOpen

      leftIdent <- identifier
      _ <- single TkColon
      leftType <- identifier

      _ <- single TkComma

      rightIdent <- identifier
      _ <- single TkColon
      rightType <- identifier

      _ <- single TkParenClose

      StOperator ident commutative leftIdent leftType rightIdent rightType <$> functionBodyStmt

    typeStmt :: ParserStmt
    typeStmt = do
      typeType <- oneOf [TkStruct]

      ident <- identifier
      _ <- single TkBraceOpen

      fields <- fieldsSection
      sections <- many section

      _ <- single TkBraceClose

      return $ composeType typeType ident (fields : sections)
      where
        section = choice [constraintSection, implSection, staticSection]

        fieldsSection = do
          fields <- many field
          return $ FieldsSection fields
          where
            field = do
              public <- isJust <$> optional (single TkPub)
              static <- isJust <$> optional (single TkStatic)
              ident <- identifier
              fieldType <- optional (single TkColon *> identifier)
              value <- optional (single (TkOp "=") *> expression)

              when (not static && isJust value) $ do
                failure Nothing (makeErrSet "non-static field with value")

              let static' = if static then Just value else Nothing

              _ <- single TkSemiColon
              return $ FruField public static' ident fieldType

        constraintSection = do
          _ <- single TkConstraintsSection
          watches <- many watch
          return $ ConstraintSection watches
          where
            watch = do
              _ <- single TkWatch
              fields <-
                between
                  (single TkParenOpen)
                  (single TkParenClose)
                  (sepBy identifier (single TkComma))
              FruWatch fields <$> blockStmt

        implSection = do
          _ <- single TkImplSection

          methods <- many method
          return $ ImplSection methods

        staticSection = do
          _ <- single TkStaticSection

          methods <- many method
          return $ StaticSection methods

        method = do
          ident <- identifier
          args <-
            between
              (single TkParenOpen)
              (single TkParenClose)
              (sepBy identifier (single TkComma))

          FruMethod ident args <$> functionBodyStmt

    expression = do
      first <- notBinaryExpr
      rest <- many ((,) <$> binaryOp <*> notBinaryExpr)

      return $ if null rest then first else ExBinaries first rest

    notBinaryExpr :: ParserExpr
    notBinaryExpr = do
      ex <- simpleExpr
      extensions <- many extensionExpr

      return $ foldl (flip ($)) ex extensions
      where
        simpleExpr :: ParserExpr
        simpleExpr =
          choice
            [ litetalNah
            , literalNumber
            , literalBool
            , literalString
            , variableExpr
            , blockExpr
            , functionExpr
            , ifElseExpr
            ]

        extensionExpr :: ParserExtExpr
        extensionExpr =
          try $
            choice
              [ callExpr
              , curryCallExpr
              , instantiationExpr
              , fieldAccessExpr
              ]

        litetalNah :: ParserExpr
        litetalNah = ExLiteralNah <$ single TkNah

        literalNumber :: ParserExpr
        literalNumber = ExLiteralNumber <$> token (\case TkNumber x -> Just x; _ -> Nothing) (makeErrSet "number")

        literalBool :: ParserExpr
        literalBool = do
          value <- token (\case TkBool x -> Just x; _ -> Nothing) (makeErrSet "bool")
          return $ ExLiteralBool value

        literalString :: ParserExpr
        literalString = do
          value <- token (\case TkString x -> Just x; _ -> Nothing) (makeErrSet "string")
          return $ ExLiteralString value

        variableExpr :: ParserExpr
        variableExpr = ExVariable <$> identifier

        functionExpr :: ParserExpr
        functionExpr = do
          _ <- single TkFn
          args <-
            between
              (single TkParenOpen)
              (single TkParenClose)
              ( sepBy
                  identifier
                  (single TkComma)
              )

          ExFunction args <$> functionBodyStmt

        ifElseExpr :: ParserExpr
        ifElseExpr = do
          _ <- single TkIf
          condition <- expression
          thenBody <- blockExpr
          _ <- single TkElse
          elseBody <- blockExpr <|> ifElseExpr

          return $ ExIfElse condition thenBody elseBody

        callExpr :: ParserExtExpr
        callExpr = do
          args <-
            between
              (single TkParenOpen)
              (single TkParenClose)
              (sepBy expression (single TkComma))
          return (`ExCall` args)

        curryCallExpr :: ParserExtExpr
        curryCallExpr = do
          args <-
            between
              (single TkDollarParenOpen)
              (single TkParenClose)
              (sepBy expression (single TkComma))
          return (`ExCurryCall` args)

        instantiationExpr :: ParserExtExpr
        instantiationExpr = do
          args <-
            between
              (single TkColonBraceOpen)
              (single TkBraceClose)
              (sepBy expression (single TkComma))
          return (`ExInstantiation` args)

        fieldAccessExpr :: ParserExtExpr
        fieldAccessExpr = do
          _ <- single TkDot
          ident <- identifier
          return (`ExFieldAccess` ident)