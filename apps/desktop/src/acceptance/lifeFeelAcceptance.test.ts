import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

describe("life feel acceptance v2 docs", () => {
  it("covers every 0270-0278 life-feel SDD with objective desktop checks", () => {
    const doc = readFileSync(
      new URL("../../../../docs/sdd/life-feel-acceptance-v2.md", import.meta.url),
      "utf8",
    );

    for (const sdd of [
      "0270",
      "0271",
      "0272",
      "0273",
      "0274",
      "0275",
      "0276",
      "0277",
      "0278",
    ]) {
      expect(doc).toContain(`SDD ${sdd}`);
    }

    expect(doc.match(/Trigger:/g)).toHaveLength(9);
    expect(doc.match(/Expected:/g)).toHaveLength(9);
    expect(doc.match(/Fail condition:/g)).toHaveLength(9);
    expect(doc.match(/Degrade check:/g)).toHaveLength(9);
    expect(doc).toContain("真实 Tauri 桌面壳");
    expect(doc).toContain("不要用浏览器替代桌面验收");
    expect(doc).toContain("不记录屏幕内容、代码正文、剪贴板或窗口标题");
  });
});
