<p align="center">
  <img src="docs/images/banner.png" alt="Rastro：放在第二块屏幕上的《逃离塔科夫》实时战局地图" width="100%">
</p>

<p align="center">
  🌐 <a href="README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.ru.md">Русский</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a> · <a href="README.fr.md">Français</a> · <a href="README.pl.md">Polski</a> · <b>中文</b>
  <br>
  <sub>社区翻译。如与<a href="README.md">英文 README</a> 有出入，以英文版为准。发现错误？<a href="https://discord.gg/H4pRxGBGDb">在 Discord 告诉我们</a>。</sub>
</p>

<p align="center">
  <a href="https://github.com/MaxSala/rastro-tarkov/releases/latest"><img src="https://img.shields.io/github/v/release/MaxSala/rastro-tarkov?label=%E6%9C%80%E6%96%B0%E7%89%88%E6%9C%AC&color=f0a43a" alt="最新版本"></a>
  <img src="https://img.shields.io/badge/%E7%8A%B6%E6%80%81-%E5%85%AC%E5%BC%80%E6%B5%8B%E8%AF%95-f0a43a" alt="公开测试">
  <a href="https://github.com/MaxSala/rastro-tarkov/stargazers"><img src="https://img.shields.io/github/stars/MaxSala/rastro-tarkov?label=Star&color=f0a43a&logo=github" alt="GitHub Star"></a>
  <a href="https://discord.gg/H4pRxGBGDb"><img src="https://img.shields.io/badge/Discord-%E5%8A%A0%E5%85%A5%E7%A4%BE%E5%8C%BA-5865F2?logo=discord&logoColor=white" alt="加入 Discord"></a>
  <img src="https://img.shields.io/badge/Windows-10%20%7C%2011-0078D4?logo=windows&logoColor=white" alt="Windows 10 和 11">
  <a href="LICENSE"><img src="https://img.shields.io/github/license/MaxSala/rastro-tarkov?color=4c4c4c" alt="MIT 许可证"></a>
</p>

<h3 align="center">
  <a href="https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe">⬇ 下载 Windows 版 Rastro</a>
</h3>

<p align="center">免费开源 · 无需安装 · 无需账号 · 无需配置</p>

---

**Rastro 是放在第二块屏幕上的实时战局地图。** 在游戏里截一张图，你的位置就会出现，并显示通往每个撤离点的路线。任务、藏身处、首领和你的小队，都在同一块屏幕上，无需 Alt+Tab。

它从不触碰游戏：只读取塔科夫本来就写入磁盘的文件。免费、开源，由玩家为玩家打造。

<p align="center">
  <img src="docs/images/hero.png" alt="海关战局中：你的位置、带路线的战局计划、首领和剩余时间。" width="100%">
  <br><sub>海关战局中：你的位置、带路线的战局计划、首领和剩余时间。</sub>
</p>

<table>
  <tr><td width="50%" valign="top"><img src="docs/images/squad.png" alt="小队一起玩"><br><b>小队一起玩</b><br>所有人在同一张地图上，PC 到 PC 端到端加密。无需服务器和账号。</td><td width="50%" valign="top"><img src="docs/images/quests.png" alt="跟随游戏的任务"><br><b>跟随游戏的任务</b><br>根据游戏日志自动勾选，每个未完成目标都显示在地图上。</td></tr>
  <tr><td width="50%" valign="top"><img src="docs/images/hideout.png" alt="藏身处"><br><b>藏身处</b><br>每个设施下一级需要什么，以及哪些物品不能卖。</td><td width="50%" valign="top"><img src="docs/images/bosses.png" alt="地图上的首领"><br><b>地图上的首领</b><br>每个首领可能出现的位置和概率，区分 PvP 与 PvE。</td></tr>
  <tr><td width="50%" valign="top"><img src="docs/images/clock.png" alt="战局时钟"><br><b>战局时钟</b><br>剩余时间、两侧的塔科夫时间和装甲列车的时间窗口。</td><td width="50%" valign="top"><img src="docs/images/market.png" alt="价格与弹药"><br><b>价格与弹药</b><br>跳蚤市场和商人价格、钥匙和所有弹药，离线可查。</td></tr>
</table>

## 安全吗？

- 只读取游戏写入磁盘的文件：截图的**文件名**，以及开启后读取的日志文件。
- 从不读取游戏内存、不模拟按键、不在游戏上方绘制任何内容。
- 只在你自己的电脑上监听（`127.0.0.1`）。没有遥测，没有账号。
- 开源（MIT）。每个版本都由 GitHub Actions 从这份代码构建。

完整解答、Windows SmartScreen 警告以及如何校验文件：[常见问题与安全](docs/zh-CN/FAQ.md)。

## 快速开始

1. **下载** [`Rastro-windows-x64.exe`](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe) 并运行，无需安装。
2. **完成简短设置。** 它会找到你的截图文件夹，并从 tarkov.dev 下载地图。
3. **在游戏里截图。** 你的箭头就会出现在地图上。

## 了解更多

**[功能](docs/zh-CN/FEATURES.md)** · **[指南：安装、快捷键和数据](docs/zh-CN/GUIDE.md)** · **[常见问题与安全](docs/zh-CN/FAQ.md)** · **[路线图](docs/ROADMAP.md)** · **[更新内容](CHANGELOG.md)** · **[参与贡献](CONTRIBUTING.md)**

> [!TIP]
> Rastro 正处于公开测试阶段，接下来做什么由社区决定。**[⭐ 给仓库点星](https://github.com/MaxSala/rastro-tarkov/stargazers)**，**[加入 Discord](https://discord.gg/H4pRxGBGDb)**：只要五秒，却很有帮助。

---

<sub>地图来自 <a href="https://github.com/the-hideout/tarkov-dev-svg-maps">tarkov.dev 贡献者</a>，CC BY-NC-SA 4.0。价格和物品数据来自 <a href="https://tarkov.dev">tarkov.dev</a>。小队截图使用的是示例位置。截图中的应用界面为英文。</sub>
<br><sub>*Escape from Tarkov* 是 Battlestate Games 的商标。Rastro 与 Battlestate Games 或 tarkov.dev 无关，也未获其认可。</sub>
<br><sub>MIT 许可证 · [致谢与第三方许可](NOTICE.md)</sub>
