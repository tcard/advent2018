// --- Part Two ---
//
// You discover a dial on the side of the device; it seems to let you select a
// square of any size, not just 3x3. Sizes from 1x1 to 300x300 are supported.
//
// Realizing this, you now must find the square of any size with the largest
// total power. Identify this square by including its size as a third parameter
// after the top-left coordinate: a 9x9 square with a top-left corner of 3,5 is
// identified as 3,5,9.
//
// For example:
//
// For grid serial number 18, the largest total square (with a total power of
// 113) is 16x16 and has a top-left corner of 90,269, so its identifier is
// 90,269,16.
//
// For grid serial number 42, the largest total square (with a total power of
// 119) is 12x12 and has a top-left corner of 232,251, so its identifier is
// 232,251,12.
//
// What is the X,Y,size identifier of the square with the largest total power?

package main

import (
	"fmt"
)

func largestSquareTotalPowerLevel(gridSide, serial int) (x, y, squareSide int) {
	var max maybeInt
	var maxX, maxY, maxSide maybeInt

	g := newGrid(gridSide, serial)

	for squareSide := 1; squareSide <= gridSide; squareSide++ {
		for x := 1; x <= gridSide-squareSide; x++ {
			for y := 1; y <= gridSide-squareSide; y++ {
				v := g.squareTotalPowerLevel(x, y, squareSide)
				if !max.some || max.v < v {
					max, maxX, maxY, maxSide = someInt(v), someInt(x), someInt(y), someInt(squareSide)
				}
			}
		}
	}

	return maxX.unwrap(), maxY.unwrap(), maxSide.unwrap()
}

type grid struct {
	side                int
	serial              int
	memoizedPowerLevel  []maybeInt
	memoizedSquareLevel []int
}

func newGrid(side, serial int) grid {
	return grid{
		side:                side,
		serial:              serial,
		memoizedPowerLevel:  make([]maybeInt, xyToScalar(side, side, side)+1),
		memoizedSquareLevel: make([]int, xyzToScalar(side, side, side, side)+1),
	}
}

func (g grid) squareTotalPowerLevel(x, y, side int) int {
	p := 0

	if side > 1 {
		// Get the previous one at the same x, y coordinates, then add the
		// missing row and column.

		p = g.memoizedSquareLevel[xyzToScalar(x, y, side-1, g.side)]
		for dx := 0; dx < side-1; dx++ {
			p += g.powerLevel(x+dx, y+side-1)
		}
		for dy := 0; dy < side-1; dy++ {
			p += g.powerLevel(x+side-1, y+dy)
		}
	}

	p += g.powerLevel(x+side-1, y+side-1)

	g.memoizedSquareLevel[xyzToScalar(x, y, side, g.side)] = p
	return p
}

func (g grid) powerLevel(x, y int) int {
	i := xyToScalar(x, y, g.side)
	if v := g.memoizedPowerLevel[i]; v.some {
		return v.v
	}
	v := powerLevel(x, y, g.serial)
	g.memoizedPowerLevel[i] = someInt(v)
	return v
}

type maybeInt struct {
	some bool
	v    int
}

func someInt(v int) maybeInt {
	return maybeInt{some: true, v: v}
}

func (v maybeInt) unwrap() int {
	if !v.some {
		panic("unwrapped empty maybeInt")
	}
	return v.v
}

func powerLevel(x, y, serial int) int {
	rackID := x + 10
	return hundredsDigit((rackID*y+serial)*rackID) - 5
}

func hundredsDigit(n int) int {
	return (n - (n / 1000 * 1000)) / 100
}

func xyToScalar(x, y, width int) int {
	return (x-1)*width + (y - 1)
}

func xyzToScalar(x, y, z, width int) int {
	return (x-1)*width*width + (y-1)*width + (z - 1)
}

func main() {
	var serial int
	_, err := fmt.Scanf("%d", &serial)
	if err != nil {
		panic(err)
	}

	x, y, squareSide := largestSquareTotalPowerLevel(300, serial)
	fmt.Printf("%d,%d,%d\n", x, y, squareSide)
}
