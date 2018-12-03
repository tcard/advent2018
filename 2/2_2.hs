-- --- Part Two ---

-- Confident that your list of box IDs is complete, you're ready to find the
-- boxes full of prototype fabric.

-- The boxes will have IDs which differ by exactly one character at the same
-- position in both strings. For example, given the following box IDs:

-- abcde
-- fghij
-- klmno
-- pqrst
-- fguij
-- axcye
-- wvxyz

-- The IDs abcde and axcye are close, but they differ by two characters (the
-- second and fourth). However, the IDs fghij and fguij differ by exactly one
-- character, the third (h and u). Those must be the correct boxes.

-- What letters are common between the two correct box IDs? (In the example
-- above, this is found by removing the differing character from either ID,
-- producing fgij.)

import System.IO (isEOF)

-- When in doubt, use brute force.

-- I also thought of a solution based on a trie, which should be
-- O(something log something) or something, but don't have time to implement it
-- in Haskell :(

sameElems :: Eq t => [t] -> [t] -> [Maybe t]
sameElems (x:xs) (y:ys) 
    | x == y = [Just x] ++ rest
    | otherwise = [Nothing] ++ rest
  where
    rest = sameElems xs ys
sameElems _ _ = []

sameElemsExceptN :: Eq t => Int -> [t] -> [t] -> Maybe [t]
sameElemsExceptN n l1 l2 =
    sameElemsExceptN_ n same []
  where 
    same = sameElems l1 l2

    sameElemsExceptN_ n ((Just x):xs) acc =
      sameElemsExceptN_ n xs (acc ++ [x])
    sameElemsExceptN_ n (Nothing:xs) acc =
      sameElemsExceptN_ (n - 1) xs acc
    sameElemsExceptN_ 0 [] acc =
      Just acc
    sameElemsExceptN_ _ _ _ =
      Nothing

sameElemsExceptOne :: Eq t => [t] -> [t] -> Maybe [t]
sameElemsExceptOne l1 l2 =
  sameElemsExceptN 1 l1 l2

findListWithSameElemsExceptOne :: Eq t => [[t]] -> Maybe [t]
findListWithSameElemsExceptOne [] = Nothing
findListWithSameElemsExceptOne (pivot:rest) =
    findListWithSameElemsThanPivotExceptOne rest
  where
    findListWithSameElemsThanPivotExceptOne [] =
      findListWithSameElemsExceptOne rest
    findListWithSameElemsThanPivotExceptOne (y:ys) =
      case (sameElemsExceptOne pivot y) of
        found @ (Just _) -> found
        Nothing -> findListWithSameElemsThanPivotExceptOne ys

main = do
    lines <- getLines
    putStrLn . show . findListWithSameElemsExceptOne $ lines
  where
    getLines = getLines_ . return $ []

    getLines_ lines = do
      done <- isEOF
      if done then do
        lines
      else do
        line <- getLine
        lines <- lines
        getLines_ . return $ line : lines
