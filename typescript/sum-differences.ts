function leftRightDifference(nums: number[]): number[] {
  let leftSum: number[] = [0];
  let rightSum: number[] = [0];
  let outPout: number[] = [];

  for (let i = 0; nums.length > i; i++) {
    leftSum.push(nums[i]);
    rightSum.unshift(nums[i]);
  }

  for (let i = 0; nums.length > i; i++) {
    leftSum[i + 2] = leftSum[i + 1] + leftSum[i + 2];
    rightSum[i + 1] = rightSum[i] + rightSum[i + 1];
  }

  for (let i = 0; i < 2; i++) {
    leftSum.splice(leftSum.length - 1, 1);
    rightSum.splice(rightSum.length - 1, 1);
  }
  rightSum.unshift(0);
  rightSum.reverse();

  for (let i = 0; nums.length > i; i++) {
    outPout.push(Math.abs(leftSum[i] - rightSum[i]));
  }

  return outPout;
}

console.log(leftRightDifference([10, 4, 8, 3]));
