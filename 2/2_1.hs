-- --- Day 2: Inventory Management System ---

-- You stop falling through time, catch your breath, and check the screen on the
-- device. "Destination reached. Current Year: 1518. Current Location: North
-- Pole Utility Closet 83N10." You made it! Now, to find those anomalies.

-- Outside the utility closet, you hear footsteps and a voice. "...I'm not sure
-- either. But now that so many people have chimneys, maybe he could sneak in
-- that way?" Another voice responds, "Actually, we've been working on a new
-- kind of suit that would let him fit through tight spaces like that. But, I
-- heard that a few days ago, they lost the prototype fabric, the design plans,
-- everything! Nobody on the team can even seem to remember important details of
-- the project!"

-- "Wouldn't they have had enough fabric to fill several boxes in the warehouse?
-- "They'd be stored together, so the box IDs should be similar. Too bad it
-- "would take forever to search the warehouse for two similar box IDs..." They
-- "walk too far away to hear any more.

-- Late at night, you sneak to the warehouse - who knows what kinds of paradoxes
-- you could cause if you were discovered - and use your fancy wrist device to
-- quickly scan every box and produce a list of the likely candidates (your
-- puzzle input).

-- To make sure you didn't miss any, you scan the likely candidate boxes again,
-- counting the number that have an ID containing exactly two of any letter and
-- then separately counting those with exactly three of any letter. You can
-- multiply those two counts together to get a rudimentary checksum and compare
-- it to what your device predicts.

-- For example, if you see the following box IDs:

-- abcdef contains no letters that appear exactly two or three times.
-- bababc contains two a and three b, so it counts for both.
-- abbcde contains two b, but no letter appears exactly three times.
-- abcccd contains three c, but no letter appears exactly two times.
-- aabcdd contains two a and two d, but it only counts once.
-- abcdee contains two e.
-- ababab contains three a and three b, but it only counts once.

-- Of these box IDs, four of them contain a letter which appears exactly twice,
-- and three of them contain a letter which appears exactly three times.
-- Multiplying these together produces a checksum of 4 * 3 = 12.

-- What is the checksum for your list of box IDs?

import qualified Data.Map.Strict as Map
import System.IO (isEOF)

countEach :: Ord t => [t] -> [Int]
countEach =
    countEach_ Map.empty
  where
    countEach_ m [] =
      Map.elems m
    countEach_ m (x:xs) =
      countEach_ (Map.insertWith (+) x 1 m) xs
    
data HasExactly2 = HasExactly2
data HasExactly3 = HasExactly3

hasExactly :: Ord t => [t] -> (Maybe HasExactly2, Maybe HasExactly3)
hasExactly list =
    (mapIfHasExactly 2 HasExactly2, mapIfHasExactly 3 HasExactly3)
  where
    mapIfHasExactly n v
      | anyHasExactly n = Just v
      | otherwise = Nothing

    anyHasExactly n =
      anyHasExactly_ n (countEach list)

    anyHasExactly_ n [] =
      False
    anyHasExactly_ n (x:xs)
      | n == x = True
      | otherwise = anyHasExactly_ n xs      

checksum :: Ord t => [[t]] -> Int
checksum =
    checksum_ (0, 0)
  where
    checksum_ (have2, have3) [] =
      have2 * have3 
    checksum_ (have2, have3) (x:xs) =
      let
        (has2, has3) = hasExactly x
        newHave2 = increaseIf has2 have2
        newHave3 = increaseIf has3 have3
      in
        checksum_ (newHave2, newHave3) xs

    increaseIf (Just _) n = n + 1
    increaseIf Nothing n = n

main = do
    lines <- getLines
    putStrLn . show . checksum $ lines
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
