function get(k, d = {}) {
  let v = localStorage.getItem(k);
  if (typeof d !== "object") {
    return v;
  }
  try {
    return JSON.parse(v || JSON.stringify(d));
  } catch (e) {
    return v;
  }
}
function set(k, v) {
  localStorage.setItem(k, typeof v === "string" ? v : JSON.stringify(v));
}
function del(k) {
  localStorage.removeItem(k);
}

export default { get, set, del };
