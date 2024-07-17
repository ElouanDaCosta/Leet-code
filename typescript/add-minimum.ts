function addMinimum(word: string): number {
  let countA = 0;
  let countB = 0;
  let countC = 0;

  // let lettersForA = 0;
  // let lettersForB = 0;
  // let lettersForC = 0;
  let lettersToAdd = 0;

  if (word === 'abc') {
    return 0;
  }

  for (let i = 0; word.length > i; i++) {
    if (word[i] === 'a') {
      countA++;
      lettersToAdd = countA * 2;
    } else if (word[i] === 'b') {
      countB++;
      lettersToAdd = countB * 2;
    } else if (word[i] === 'c') {
      countC++;
      lettersToAdd = countC * 2;
    } else if (word[i] === 'ab') {
      lettersToAdd += 1;
    } else if (word[i] === 'bc') {
      lettersToAdd += 1;
    } else if (word[i] === 'ac') {
      lettersToAdd += 1;
    }
  }

  return lettersToAdd;
}

console.log(addMinimum('aaaaab'));
