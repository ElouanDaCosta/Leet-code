// Link to the exercice https://leetcode.com/problems/merge-strings-alternately/description/

function mergeAlternately(word1: string, word2: string): string {
  let wordArray1: string[];
  let wordArray2: string[];
  let wordOutput: string[];
  let output: string;

  wordArray1 = word1.split('');
  wordArray2 = word2.split('');
  wordOutput = [];

  if (word1.length >= word2.length) {
    for (let i = 0; word1.length > i; i++) {
      wordOutput.push(wordArray1[i]);
      wordOutput.push(wordArray2[i]);
    }
  } else if (word1.length <= word2.length) {
    for (let i = 0; word2.length > i; i++) {
      wordOutput.push(wordArray1[i]);
      wordOutput.push(wordArray2[i]);
    }
  }
  output = wordOutput.join('');
  return output;
}

mergeAlternately('ab', 'pqrs');
