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

<p align="center">
  免费开源 · 无需安装 · 无需账号 · 无需配置
  <br>
  <a href="#工作原理">工作原理</a> ·
  <a href="#和小队一起玩">小队模式</a> ·
  <a href="#安全吗">安全吗？</a> ·
  <a href="#安装">安装</a> ·
  <a href="#rastro-的发展方向">路线图</a> ·
  <a href="#参与进来">参与进来</a> ·
  <a href="https://discord.gg/H4pRxGBGDb">Discord</a>
</p>

> [!TIP]
> **Rastro 正处于公开测试阶段，而这只是开始。** 任务、藏身处以及更多功能都在路上，先做什么由社区决定。
> 如果你喜欢它的方向，请 **[⭐ 给仓库点个 Star](https://github.com/MaxSala/rastro-tarkov/stargazers)** 并 **[加入 Discord](https://discord.gg/H4pRxGBGDb)**。只要五秒钟，帮助却比你想象的大。

---

**Rastro 是一张运行在第二块屏幕上的实时战局地图。** 在游戏里截一张图，你的位置就会出现在地图上，并显示到每个撤离点的距离和方向。开一个房间，整个小队就能在同一张地图上看到彼此，每个人用自己的电脑。把鼠标移到一扇锁着的门上，就能看到哪把钥匙能开、要花多少钱。查价格、查弹药，都不用 Alt+Tab 切到浏览器。

它从不碰游戏。Rastro 只读取塔科夫本来就会写到你硬盘上的文件，而且免费、开源，由玩家为玩家打造。

## 截图

<table>
  <tr>
    <td width="50%"><img src="docs/images/party.png" alt="海关地图上的三名小队成员，显示其中一名队友的信息卡"></td>
    <td width="50%"><img src="docs/images/map.png" alt="海关地图，显示你的位置、轨迹和前往撤离点的导航"></td>
  </tr>
  <tr>
    <td align="center"><b>小队。</b> 所有人的位置和朝向都在同一张地图上，还能标点。中间没有任何服务器。</td>
    <td align="center"><b>地图。</b> 你的位置和轨迹、所有撤离点，以及到所选撤离点的实时距离和方向。</td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/images/floors.png" alt="宿舍二楼，地图其余部分和楼下的队友被调暗"></td>
    <td width="50%"><img src="docs/images/keys.png" alt="门锁图层，打开了一张钥匙信息卡"></td>
  </tr>
  <tr>
    <td align="center"><b>楼层。</b> 跟随你在楼层间移动，把不在你这层的东西都调暗。</td>
    <td align="center"><b>钥匙。</b> 地图上每扇锁着的门、能打开它的钥匙以及钥匙的价格。</td>
  </tr>
  <tr>
    <td width="50%"><img src="docs/images/market.png" alt="市场搜索，显示某个物品的报价"></td>
    <td width="50%"><img src="docs/images/ammo.png" alt="某个口径的弹药表"></td>
  </tr>
  <tr>
    <td align="center"><b>市场。</b> 即时离线搜索，包含跳蚤市场和商人价格，可按每格价值排序。</td>
    <td align="center"><b>弹药。</b> 每种子弹的伤害、穿透和护甲伤害，以及最便宜的购买途径。</td>
  </tr>
</table>

<sub>地图来自 <a href="https://github.com/the-hideout/tarkov-dev-svg-maps">tarkov.dev 贡献者</a>，CC BY-NC-SA 4.0。价格和物品数据来自 <a href="https://tarkov.dev">tarkov.dev</a>。小队截图使用的是示例位置。截图中的应用界面为英文。</sub>

## Rastro 的发展方向

Rastro 最初只是一张放在第二块屏幕上、让你不再迷路的地图。我们的计划远不止于此：**做一个陪伴你的应用，涵盖你在战局之间和战局中要查的一切**，并且依旧从不碰游戏。目前计划中的有：

| | 即将推出 | 对你意味着什么 |
|---|---|---|
| 📜 | **任务追踪** | 你正在进行的任务显示在地图上，每个目标都有标记。新手有逐步指引，老手有快速清单。 |
| 👥 | **小队任务** | 在同一张地图上看到队友未完成的任务，让整个小队规划出一条共同路线。 |
| 🏠 | **藏身处追踪** | 每项升级、所需材料以及该留下哪些物品。别再把明天要用的东西今天卖掉。 |
| 🔑 | **你的钥匙串** | 标记你拥有的钥匙，能打开的门会在每张地图上高亮显示。 |
| 🛒 | **购物清单** | 收藏物品，价格变动时收到提醒。 |
| 📊 | **战局记录** | 每场战局的地图、时长、排队时间和游戏模式。 |
| 🎯 | **弹药对护甲** | 每种子弹对每个护甲等级的穿透图表。 |
| 📍 | **分享标点** | 把你的物资点和藏匿点发给小队，或者转移到另一台电脑。 |

这些都还没有具体日期，顺序也不是固定的：**由玩家的需求决定。** 完整列表（包括较小的改进和已发布的内容）见 [docs/ROADMAP.md](docs/ROADMAP.md)（英文）。

**有想法？** 发到 [Discord](https://discord.gg/H4pRxGBGDb) 的 feature requests 论坛，或者[提交 issue](https://github.com/MaxSala/rastro-tarkov/issues)。每条建议我们都会看，呼声最高的会优先安排。

## 工作原理

Rastro 读取《逃离塔科夫》本来就会保存在你电脑上的两类文件，除此之外什么都不需要。

| 读取什么 | 位置 | Rastro 从中获得什么 |
|---|---|---|
| **截图的文件名** | `Documents\Escape from Tarkov\Screenshots` | 你的位置和朝向。塔科夫会把你的坐标写进每张截图的文件名里。Rastro 只读文件名，从不读图片内容，也从不移动、修改或删除你的截图。 |
| **游戏日志**（可选，默认关闭，需要你手动开启） | `Battlestate Games\Escape from Tarkov\Logs` | 正在加载哪张地图、战局何时开始，以及你在玩 PvP 还是 PvE。Rastro 据此自动切换到正确的地图，每场战局开始新的轨迹，并显示剩余战局时间。只保存地图名称和游戏模式。 |

实际使用时：你在游戏里按下截图键，一秒后你的箭头就会在地图上移动。

Rastro **不会**做的事：不读取游戏内存，不注入任何东西，不挂钩游戏进程，不抓取网络流量，不在游戏画面上绘制任何内容，也不会替你按键。它是一个独立程序，只查看硬盘上的文件。

## 和小队一起玩

Rastro 的小队模式让整个小队出现在同一张地图上，每个人都有自己的名字和颜色。

- **看到每个人的位置。** 每位队友的箭头显示他最后一次截图时所在的位置和朝向。如果有人在别的楼层，他的箭头会变暗，鼠标移上去就能看到他在几楼。
- **标点。** `Shift` + 点击地图上任意位置，这个地方就会在所有人的屏幕上闪烁。
- **设置简单。** 一名玩家创建房间并复制邀请码，其他人把它粘贴到 *加入* 里，就这么简单。最多 8 名玩家。
- **无服务器，无账号。** 位置数据直接在电脑之间传输。除了你们，没人能看到，包括我们。
- **加密。** 所有流量都使用 [Noise 协议](https://noiseprotocol.org/)端到端加密。邀请码就是密钥，所以只分享给你的小队。

其他人需要能连接到房主的电脑。最简单的办法是使用免费的组网 VPN，比如 [Tailscale](https://tailscale.com/) 或 [ZeroTier](https://www.zerotier.com/)：每个人安装一次，Rastro 就能像在同一个局域网里一样工作，不用在路由器上开端口。在房主的路由器上开放端口也可以。

每位玩家仍然在自己的电脑上运行 Rastro 并自己截图：正是这样才能让每个人的位置保持最新。

## 功能

**地图**
- 每次截图都会更新你的位置和朝向，并留下一条逐渐淡出的轨迹。
- 点击撤离点或任意位置，查看距离、该往哪边转以及高度差。
- 撤离点可按 PMC、Scav 或共用筛选。勾选游戏提示本局开放的撤离点，其余的会被调暗。
- 来自 tarkov.dev 的图层：转移点、开关、物资容器（可按类型筛选）和门锁。
- 多层地图会自动跟随你切换楼层，也可以在楼层面板中手动选择。地图的其余部分以及其他楼层上的内容都会被调暗。
- 鼠标移到任何东西上都会显示信息卡：是什么、有多远、在哪一层以及点击后会发生什么。
- 自定义标点：在地图上右键保存物资点、危险区域、藏匿点或任务地点。标点以游戏坐标保存，重新安装地图也不会丢失。

**钥匙**
- *门锁* 图层显示每一扇需要钥匙的门、每一个容器和汽车后备箱。需要通电的锁带有黄色边框。
- 点击一把锁，查看能打开它的钥匙、在哪里买最便宜以及最高卖价。
- *高亮这把钥匙* 会调暗其他所有锁，让你看到一把钥匙能打开的全部地方。
- 市场里的每把钥匙都会列出它在哪些已安装的地图上使用，并附带直接跳转的链接。

**市场**
- 按名称或简称搜索完整的物品目录，不区分重音和词序。按 `/` 开始搜索，用方向键浏览结果。
- 可按最高卖价、跳蚤市场价格、24 小时均价、48 小时涨跌或每格价格排序。
- 每个物品都列出所有商人和跳蚤市场的报价，包括所需的商人等级以及原始的美元或欧元价格。
- 目录只需下载一次，在你的电脑上搜索，所以搜索是即时的，离线也能用。

**弹药**
- 所有子弹按口径分组，并按穿透排序。
- 伤害（霰弹为单颗弹丸伤害 × 弹丸数）、穿透、护甲伤害、碎裂概率、初速以及最便宜的购买途径。曳光弹有标记。

**战局检测**（可选）
- 战局加载时切换到正确的地图，每场战局开始时清除轨迹和标记，并显示剩余时间。它读取游戏日志，详见[工作原理](#工作原理)。

**其他**
- 16 种语言，与 tarkov.dev 支持的相同：Čeština、Deutsch、English、Español、Français、Magyar、Italiano、日本語、한국어、Polski、Português、Română、Русский、Slovenčina、Türkçe 和中文。物品、地图和撤离点名称也会跟随你的语言。
- PvP 和 PvE 数据，可根据游戏日志自动选择，也可手动选择。
- 可选的更新提醒，每次更新后显示 *新功能* 页面。
- 所有功能都有快捷键，还有命令面板（`Ctrl+K`）。

## 安全吗？

Rastro 是一个小巧的便携程序。以下是这意味着什么，以及你如何自行验证。

- **无需安装，无需管理员权限。** 它只是一个 `.exe` 文件。不安装服务或驱动，不随 Windows 启动，也不碰游戏文件。设置和下载的地图都放在一个文件夹里（见[数据存放位置](#数据存放位置)），删掉这个文件夹和 `.exe` 就能彻底移除。
- **开源。** 所有代码都在这个仓库里，任何人都可以查看。
- **公开构建。** 每个版本都由 GitHub 自己的服务器直接从源码构建，而不是在某个人的电脑上。你可以在 [Actions](https://github.com/MaxSala/rastro-tarkov/actions) 标签页看到每次构建，发布页面也会列出文件的 SHA-256，方便你确认下载的正是这个文件：

  ```powershell
  Get-FileHash .\Rastro-windows-x64.exe
  ```

- **不会偷偷上传。** 没有遥测，没有广告，没有账号。只有你主动要求时才会联网，完整的连接列表见[隐私](#隐私与网络)。

**“Windows 已保护你的电脑”。** 第一次打开 Rastro 时，Windows SmartScreen 很可能会弹出一个蓝色窗口，说它阻止了一个无法识别的应用。任何没有付费证书签名的新程序都会这样，并不是文件有问题。点击 **更多信息**，然后点击 **仍要运行**。Windows 会记住你的选择。有些杀毒软件也会对没有签名的新程序比较警惕。如果你的杀毒软件报了 Rastro，你可以恢复文件并添加例外，把文件上传到 [VirusTotal](https://www.virustotal.com/) 获取第二意见，并向杀毒软件厂商报告误报。

> [!IMPORTANT]
> Rastro 是非官方的粉丝项目，与 Battlestate Games 或 tarkov.dev 没有任何关联，也未获得其认可。它的设计原则是远离游戏：只读取硬盘上的文件，从不与游戏进程交互。尽管如此，根据 Battlestate Games 的服务条款，在《逃离塔科夫》中使用第三方工具的责任由你自行承担。

## 安装

1. **[下载 `Rastro-windows-x64.exe`](https://github.com/MaxSala/rastro-tarkov/releases/latest/download/Rastro-windows-x64.exe)**。这个链接始终指向最新版本。
2. 放在任何地方都行：桌面、文档里的某个文件夹，随你喜欢。它旁边不需要任何其他文件。
3. 双击运行。如果 Windows 显示蓝色警告，见[安全吗？](#安全吗)

需要 64 位 Windows 10 或 11。

### 首次启动

第一次运行时会打开一个简短的引导：

1. **截图文件夹。** Rastro 会自动找到 `Documents\Escape from Tarkov\Screenshots`，即使 Windows 把文档移到了 OneDrive 也没问题。如果找不到，粘贴路径后它会告诉你看到了多少张带坐标的截图。你也可以在这里开启战局检测。
2. **地图。** 从 tarkov.dev 下载社区地图（推荐）、导入地图包，或者试试演示地图。
3. 完成。在游戏里截一张图，你的箭头就会出现在地图上。

要使用市场、弹药表和钥匙名称，请先打开一次市场标签页并点击 *下载价格*。

### 更新

如果你允许了检查更新，有新版本时顶部栏会出现一个按钮，带你到下载页面。否则，就通过[上面的链接](#安装)下载新的 `.exe`，关闭 Rastro，用新文件替换旧文件。你的设置、标点、地图和价格都[单独保存](#数据存放位置)，不会丢失。

更新后，Rastro 会告诉你改了什么。完整历史见 *设置 → 关于 → 新功能* 和 [CHANGELOG.md](CHANGELOG.md)。如果更新带来了新的地图数据，请在地图面板中使用 *从 tarkov.dev 更新* 获取。

### 快捷键

| 操作 | 方式 |
|---|---|
| 更新你的位置 | 在游戏中截图 |
| 切换版块（地图、市场、弹药） | `Ctrl+1`、`Ctrl+2`、`Ctrl+3` |
| 命令面板 | `Ctrl+K` |
| 撤离点、标点、小队和地图面板 | `1`、`2`、`3`、`4` |
| 新战局（清除轨迹和标记） | `N` |
| 查看整张地图 / 跟随你的位置 | `F` / `G` |
| 缩放 | `+` / `-` 或鼠标滚轮 |
| 添加标点、导航到某处 | 在地图上右键 |
| 给小队标点 | `Shift` + 点击 |
| 搜索市场 | `/` |
| 设置 | `,` |

### 数据存放位置

设置、你的标点、下载的地图和价格缓存都放在一个文件夹里，与 `.exe` 分开：

| 系统 | 文件夹 |
|---|---|
| Windows | `%APPDATA%\Rastro\data\` |
| Linux | `~/.local/share/rastro/` |
| macOS | `~/Library/Application Support/Rastro/` |

*设置 → 日志* 里有一个打开它的按钮。

### 遇到问题

- **在浏览器里打开，而不是自己的窗口。** Rastro 使用 Microsoft WebView2 绘制窗口。Windows 10 和 11 自带它，但如果被删除了，Rastro 会改用你的浏览器打开。安装 [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/) 后窗口就会恢复。
- **错误页面提示端口已被占用。** 可能还有另一个 Rastro 在运行。从任务栏或任务管理器关闭它（查找 `rastro.exe`），再重新打开。
- **我的位置一直不动。** 检查 *设置 → 常规* 里的截图文件夹是否就是游戏实际保存截图的位置，以及你按的是不是游戏内的截图键。
- **队友进不了我的房间。** 他们需要能连接到你的电脑。创建房间时使用组网 VPN（Tailscale、ZeroTier）提供的地址，或者在路由器上开放小队端口。
- **还是不行？** 到 [Discord](https://discord.gg/H4pRxGBGDb) 提问。*设置 → 日志* 里有 *复制* 按钮，贴出这份日志会很有帮助。

## 常见问题

**用它会被封号吗？**
Rastro 从不读取游戏内存，从不碰游戏进程，也从不向游戏发送任何输入。它只在游戏写完之后读取截图文件名和日志文件，这些文件你自己也可以在资源管理器里打开。不过我们无法代表 Battlestate Games 表态，请参阅[安全吗？](#安全吗)中的说明。

**PvE 能用吗？**
能。战局检测已用 PvE 日志测试过，Rastro 检测到 PvE 时会下载 PvE 的价格和数据。

**需要联网吗？**
只有下载地图和价格、检查更新（如果你允许）以及小队联机时需要。其他功能都能离线使用。

**为什么用截图？**
截图是游戏把你的精确位置写到硬盘上的唯一地方。用几秒钟的延迟，换来永远不碰游戏。

**支持 Linux 或 macOS 吗？**
Rastro 可以在 Linux 上构建和运行；macOS 未经测试。塔科夫本身只能在 Windows 上运行，所以只提供 Windows 版下载。见[从源码构建](#从源码构建)。

## 隐私与网络

Rastro 没有账号、没有遥测、没有广告。界面只与运行在 `127.0.0.1` 上的应用本身通信，并由每次启动都会变化的随机密钥保护。它从你硬盘读取的所有内容都留在你的电脑上。其他一切只在你要求时才会发生：

| 何时 | 连接到 |
|---|---|
| 始终 | 不连接你电脑之外的任何地方 |
| 你从 tarkov.dev 下载地图 | `raw.githubusercontent.com`、`json.tarkov.dev`，备用 `api.tarkov.dev` |
| 你下载或刷新价格 | `json.tarkov.dev`，以及用于物品图标的 `assets.tarkov.dev` |
| 你从 URL 导入地图包 | 只连接你输入的地址（仅 HTTPS，从不连接内网地址） |
| 你创建或加入小队 | 直接连接队友的电脑 |
| 每次启动一次（如果你同意检查更新），或点击 *立即检查* 时 | `api.github.com`，读取最新版本号。不会发送任何关于你的信息。 |

每个下载服务器都在固定的白名单上，重定向时也会检查，所有下载的内容在使用前都会经过校验。完整的威胁模型见 [docs/SECURITY.md](docs/SECURITY.md)（英文）。

## 参与进来

Rastro 由一个小团队公开开发，我们希望和使用它的人一起把它做大。不管你每周有五秒钟还是五个小时，都有办法帮上忙。

### ⭐ 支持项目

- **给仓库点个 Star。** 免费，只需点一下，这也是新玩家在 GitHub 上发现 Rastro 的主要方式。
- **Watch → Custom → Releases**，新版本发布时就会收到通知。
- **告诉你的小队。** 小队模式和朋友一起玩更好，大多数人都是通过口口相传找到我们的。
- **分享 Rastro 实战的片段或截图**，并在 Discord 上 @ 我们。

### 💬 加入社区

**[Rastro Discord](https://discord.gg/H4pRxGBGDb)** 是一切发生的地方：版本公告、实时路线图、其他玩家的帮助、Bug 和建议论坛，还能找队友一起试玩小队模式。日常开发都在那里进行，所以那是第一时间看到新内容的最佳去处。

### 🙋 我们在招人

我们的想法远比人手多。如果你符合下面任何一个角色，非常欢迎联系我们：

| 角色 | 你会做什么 |
|---|---|
| 🛡️ **管理员** | 在社区成长的过程中，让 Discord 保持友好、有用。 |
| 🌍 **翻译** | 校对尚未经过母语者检查的 14 种语言之一，或持续维护你的语言。 |
| 🦀 **开发者** | 后端用 Rust，界面用 React 和 TypeScript。从小修复到完整功能都可以。 |
| 🧪 **测试** | 抢先试用新版本，尤其是 PvP、冷门地图和不同的电脑配置。 |
| 🗺️ **地图与数据** | 对照游戏核对位置、楼层和门锁，并协助处理 tarkov.dev 尚未覆盖的地图。 |
| 🎥 **内容创作者** | 想展示 Rastro 或反馈自己社区意见的主播和视频作者。 |

**如何申请：** 在 [Discord](https://discord.gg/H4pRxGBGDb) 的支持频道开一个 *Join the team* 工单。开发者和翻译也可以直接提交 pull request：方法见 [CONTRIBUTING.md](CONTRIBUTING.md)（英文）。

### 💡 想法、反馈与 Bug

真诚的反馈是你能给测试版最宝贵的东西。告诉我们哪里让你困惑、缺了什么、什么功能你每局都会用：

- **想法：** [Discord](https://discord.gg/H4pRxGBGDb) 的 feature requests 论坛，或 [GitHub issue](https://github.com/MaxSala/rastro-tarkov/issues)。
- **Bug：** Discord 的 bug reports 论坛，或 GitHub issue。请附上你的 Rastro 版本（*设置 → 关于*）以及你当时在做什么。请勿公开发布完整的游戏日志，因为里面包含你的账号 ID。
- **安全问题：** 绝不要公开发布。请使用 GitHub 的私密报告（*Security* 标签页 → *Report a vulnerability*），或在 Discord 开一个私密的 *Security & privacy* 工单。详情见 [docs/SECURITY.md](docs/SECURITY.md)。

## 致谢

没有 **[tarkov.dev](https://tarkov.dev)** 就没有 Rastro。tarkov.dev 是由 **[The Hideout](https://github.com/the-hideout)** 社区维护的开源《逃离塔科夫》数据库，最初是 **Oskar Risberg（[@kokarn](https://github.com/kokarn)）** 的 [tarkov-tools](https://github.com/kokarn/tarkov-tools)，后来由 The Hideout 的贡献者重新启动并向所有人开放。Rastro 使用了：

- **地图：** 来自 [the-hideout/tarkov-dev-svg-maps](https://github.com/the-hideout/tarkov-dev-svg-maps) 的 SVG 地图，作者为 tarkov.dev 的贡献者，采用 [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) 许可。作者不允许将地图用于作弊或提供不公平优势的软件（例如雷达）。地图由你自己下载，从不随 Rastro 一起分发，每张已安装的地图都保留其署名和许可证。
- **数据：** 撤离点、转移点、开关、容器、门锁、物品、价格和翻译，来自 tarkov.dev API 及其数据文件。
- **标记图标：** 来自 [tarkov-dev](https://github.com/the-hideout/tarkov-dev)（`public/maps/interactive`），MIT 许可证，© 2019 Oskar Risberg。
- **物品图标** 归 Battlestate Games 所有，仅在本地缓存用于显示。

如果 Rastro 对你有用，请考虑支持 tarkov.dev、为它做贡献或加入他们的社区。

Rastro 还基于许多其他开源项目，包括 Rust、Tokio、Axum、Snow、Rustls、Wry、React、Leaflet、Tailwind CSS 和 Geist 字体。完整列表及许可证见 [NOTICE.md](NOTICE.md)。

*Escape from Tarkov*（《逃离塔科夫》）是 Battlestate Games 的商标。游戏内容和素材是 Battlestate Games 及其许可方的商标和版权作品。

## 非商业项目

Rastro 是为社区制作的业余项目。它免费、没有付费版本、不显示广告、不收集数据。它会一直保持这样，这既是我们的选择，也因为它所依赖的 tarkov.dev 地图仅限非商业用途。

## 已知限制

- 只有截图时位置才会更新。这是有意为之：截图是游戏写入你位置的唯一地方。
- 实验室在 tarkov.dev 上没有 SVG 地图。Rastro 能识别它，但除非你导入自己的地图包，否则无法显示。
- 战局检测已用 PvE 日志验证。PvP 应该也一样，但尚未用真实日志确认。
- 西班牙语和英语翻译已经过校对。其余 14 种尚未经过母语者检查。非常欢迎在这里或 [Discord](https://discord.gg/H4pRxGBGDb) 上提出修正。
- 切换语言或游戏模式（PvP/PvE）后，需要重新下载地图和价格才能看到新名称。
- 无论选择哪种语言，错误信息和日志中的部分技术细节仍为英文。

## 从源码构建

只有想修改代码或自己构建 `.exe` 时才需要这一步。你需要 [Rust](https://rustup.rs/) 1.85 或更新版本，以及 [Node.js](https://nodejs.org/) LTS（Linux 上还需要 `libwebkit2gtk-4.1-dev` 和 `libgtk-3-dev`）：

```sh
git clone https://github.com/MaxSala/rastro-tarkov.git
cd rastro-tarkov
./build.sh        # Windows 上用 build.bat
```

构建结果是 `target/release/` 下的单个可执行文件。开发环境配置、项目结构、代码规范以及如何添加翻译，见 **[CONTRIBUTING.md](CONTRIBUTING.md)**（英文）。

## 许可证

Rastro 的源代码以 [MIT 许可证](LICENSE)发布。从 tarkov.dev 下载的地图和数据保留各自的许可证，列于[致谢](#致谢)和 [NOTICE.md](NOTICE.md) 中。

---

<p align="center">
  <b>如果 Rastro 帮你找到了出口，<a href="https://github.com/MaxSala/rastro-tarkov/stargazers">请给它一个 ⭐</a>，也欢迎<a href="https://discord.gg/H4pRxGBGDb">来 Discord 打个招呼</a>。</b>
  <br>
  <sub>由玩家打造，为玩家服务。撤离点见。</sub>
</p>
