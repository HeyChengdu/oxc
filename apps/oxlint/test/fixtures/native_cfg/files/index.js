function choose(flag) {
  if (flag) return "正常";
  throw new Error("异常");
}

choose(true);
