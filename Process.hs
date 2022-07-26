import Data.Char  (isSpace)

main :: IO ()
main = putStrLn =<< fmap go1 getContents
  where
    go1 :: String -> String
    go1 = go2 . span (not . isSpace) . dropWhile isSpace
    
    go2 :: (String, String) -> String
    go2 ("", "")       = ""
    go2 (testName, xs) = let (results, ys) = span (/= ']') . dropWhile (/= '[') $ xs
                         in  take (77 - length results) (testName ++ replicate 78 ' ') ++ results ++ "]\n" ++ go1 (drop 1 ys)
