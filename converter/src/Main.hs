module Main
  ( main
  ) where

import Control.Monad (unless, when)
import Data.Aeson (encode, object, (.=))
import Data.Aeson.Key (fromString)
import Data.ByteString.Lazy.Char8 (unpack)
import Data.Either (fromLeft, fromRight, isLeft)
import Data.List (intercalate)
import Debugize (toDbgStrStmt)
import Jsonize (toJsonStmt, toString)
import System.Environment (getArgs)
import System.Exit (exitFailure)
import Text.Megaparsec (parse)
import Tokenize (fruTokenize)
import Treeanize (toAst)


possibleFlags :: [String]
possibleFlags = ["--debug"]


main :: IO ()
main = do
  allAarguments <- getArgs
  when (null allAarguments) $ do
    print "No source files specified"
    exitFailure

  let filename = head allAarguments
  let arguments = tail allAarguments
  let unknownFlags = filter (`notElem` possibleFlags) arguments

  unless (null unknownFlags) $ do
    print $ "Unknown flags: " ++ intercalate ", " unknownFlags
    exitFailure

  let debugFlag = "--debug" `elem` arguments

  rawFile <- readFile filename

  -- tokens --
  let tokensOrError = parse fruTokenize filename rawFile
  when (isLeft tokensOrError) $ do
    let tokenizingError = fromLeft undefined tokensOrError

    when debugFlag $ do
      putStrLn "---------- ERROR WHILE TOKENIZING ----------"
      print tokensOrError

    unless debugFlag $ do
      (putStrLn . unpack . encode . object)
        [ fromString "error" .= "tokenizing"
        , fromString "message" .= show tokenizingError
        ]
    exitFailure

  let tokens = fromRight undefined tokensOrError

  when debugFlag $ do
    putStrLn "---------- TOKENS ----------"
    putStrLn (intercalate "\n" $ map (("| " ++) . show) tokens)

  -- ast --
  let astOrError = parse toAst filename tokens
  when (isLeft astOrError) $ do
    let treeanizingError = fromLeft undefined astOrError

    when debugFlag $ do
      putStrLn "---------- ERROR WHILE TREEANIZING ----------"
      print astOrError

    unless debugFlag $ do
      (putStrLn . unpack . encode . object)
        [ fromString "error" .= "treeanizing"
        , fromString "message" .= show treeanizingError
        ]
    exitFailure

  let ast = fromRight undefined astOrError

  when debugFlag $ do
    putStrLn "---------- AST ------------"
    putStrLn (toDbgStrStmt 0 ast)

  unless debugFlag $ do
    putStrLn (toString $ toJsonStmt ast)
