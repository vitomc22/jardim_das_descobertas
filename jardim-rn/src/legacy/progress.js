/* Daily activity progress, based on the device's local calendar. MIT. */
(function (root) {
  "use strict";
  const GOAL = 5;
  function localDay(now = new Date()) {
    if (!(now instanceof Date) || !Number.isFinite(now.getTime()))
      throw new RangeError("Invalid local date");
    return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, "0")}-${String(now.getDate()).padStart(2, "0")}`;
  }
  function nextMidnight(now = new Date()) {
    return new Date(
      now.getFullYear(),
      now.getMonth(),
      now.getDate() + 1,
    ).getTime();
  }
  function normalise(raw, now = new Date(), validKeys) {
    const date = localDay(now),
      out = { date, activities: {} };
    if (
      !raw ||
      raw.date !== date ||
      !raw.activities ||
      typeof raw.activities !== "object" ||
      Array.isArray(raw.activities)
    )
      return out;
    for (const [key, value] of Object.entries(raw.activities)) {
      if (
        !/^(little|middle|big):[a-z]+$/.test(key) ||
        (validKeys && !validKeys.has(key)) ||
        !value ||
        typeof value !== "object"
      )
        continue;
      const solved =
        Number.isSafeInteger(value.solved) && value.solved >= 0
          ? Math.min(GOAL, value.solved)
          : 0;
      out.activities[key] = { solved, completed: solved === GOAL };
    }
    return out;
  }
  function status(daily, key) {
    const record = daily.activities[key];
    return record
      ? { solved: record.solved, completed: record.solved >= GOAL }
      : { solved: 0, completed: false };
  }
  function record(daily, key) {
    if (!/^(little|middle|big):[a-z]+$/.test(key))
      throw new Error("Invalid activity key");
    const solved = Math.min(GOAL, status(daily, key).solved + 1);
    daily.activities[key] = { solved, completed: solved === GOAL };
    return status(daily, key);
  }
  function summary(daily, keys) {
    const unique = [...new Set(keys)],
      completed = unique.filter((k) => status(daily, k).completed).length;
    return {
      completed,
      total: unique.length,
      percent: unique.length
        ? Math.round((completed / unique.length) * 100)
        : 0,
    };
  }
  const api = {
    GOAL,
    localDay,
    nextMidnight,
    normalise,
    status,
    record,
    summary,
  };
  if (typeof module !== "undefined") module.exports = api;
  else root.DailyProgress = api;
})(typeof window !== "undefined" ? window : globalThis);
