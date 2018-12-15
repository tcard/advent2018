// --- Part Two ---

// As it turns out, you got the Elves' plan backwards. They actually want to
// know how many recipes appear on the scoreboard to the left of the first
// recipes whose scores are the digits from your puzzle input.

// 51589 first appears after 9 recipes.
// 01245 first appears after 5 recipes.
// 92510 first appears after 18 recipes.
// 59414 first appears after 2018 recipes.

// How many recipes appear on the scoreboard to the left of the score sequence
// in your puzzle input?

import 'dart:io';
import 'dart:convert';

typedef Combine = Iterable<int> Function(Iterable<int>);

Iterable<int> scores(Iterable<int> initial, Combine combine) sync* {
  var scores = <int>[];
  var positions = <int>[];
  for (var elem in enumerate(initial)) {
    scores.add(elem.value);
    yield elem.value;
    positions.add(elem.index);
  }

  while (true) {
    var selected = positions.map((i) => scores[i]);
    for (var newScore in combine(selected)) {
      scores.add(newScore);
      yield newScore;
    }

    for (var i = 0; i < positions.length; i++) {
      positions[i] = (positions[i] + 1 + scores[positions[i]]) % scores.length;
    }
  }
}

class Enumerated<T> {
  int index;
  T value;

  Enumerated(this.index, this.value);
}

Iterable<Enumerated<T>> enumerate<T>(Iterable<T> iterable) sync* {
  var i = 0;
  for (var elem in iterable) {
    yield Enumerated(i, elem);
    i++;
  }
}

T sum<T extends num>(Iterable<T> nums) {
  return nums.reduce((sum, next) => sum + next);
}

Iterable<int> digits(int n) sync* {
  if (n == 0) {
    yield 0;
    return;
  }
  while (n > 0) {
    yield n % 10;
    n = n ~/ 10;
  }
}

Iterable<int> occurrences<T>(List<T> of, Iterable<T> inside) sync* {
  var window = inside.take(of.length).toList();
  var iter = inside.skip(of.length).iterator;
  for (var i = 0; ; i++) {
    if (sameElems(of, window)) {
      yield i;
    }

    if (!iter.moveNext()) {
      break;
    }

    window.removeAt(0);
    window.add(iter.current);
  }
}

bool sameElems<T>(List<T> a, List<T> b) {
  if (a.length != b.length) {
    return false;
  }
  for (var i = 0; i < a.length; i++) {
    if (a[i] != b[i]) {
      return false;
    }
  }
  return true;
}

main() {
  var toFind = int.parse(stdin.readLineSync());

  print(
    occurrences(
      digits(toFind).toList().reversed.toList(),
      scores([3, 7], (scores) => digits(sum(scores)).toList().reversed),
    ).first
  );
}
