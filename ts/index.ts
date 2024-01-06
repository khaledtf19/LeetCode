function topKFrequent(nums: number[], k: number): number[] {
  let hm = new Map<number, number>();
  nums.forEach((num) => {
    const r = hm.get(num);
    if (r) {
      hm.set(num, r + 1);
    } else {
      hm.set(num, 1);
    }
  });

  return [...hm.entries()]
    .sort((a, b) => b[1] - a[1])
    .slice(0, k)
    .map(([key, _]) => key);
}

let res = topKFrequent([3, 0, 1, 0], 1);
