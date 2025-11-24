# ECL 网站部署指南

## GitHub Pages 部署步骤

### 1. 准备仓库
1. 在 GitHub 上创建一个新的仓库，或者使用现有的仓库
2. 将所有网站文件提交到仓库的主分支

### 2. 启用 GitHub Pages
1. 进入仓库的 Settings 页面
2. 找到 "Pages" 部分
3. 在 "Source" 下拉菜单中选择 "Deploy from a branch"
4. 选择 "main" 分支和 "/ (root)" 文件夹
5. 点击 "Save"

### 3. 配置仓库
确保你的仓库包含以下文件：
```
/
├── index.html              # 主页
├── website/
│   ├── docs_reader.html   # 文档阅读器
│   ├── style.css          # 样式文件
│   ├── script.js          # 脚本文件
│   └── test.html          # 测试页面（可选）
├── docs/                  # Markdown文档目录
│   ├── introduction.md
│   ├── quickstart.md
│   ├── syntax.md
│   └── ...（其他文档）
├── _config.yml            # Jekyll配置文件
└── DEPLOYMENT.md          # 部署说明
```

### 4. 访问网站
部署完成后，你的网站将在以下地址访问：
```
https://[你的用户名].github.io/[仓库名]/
```

## 本地测试

### 方法一：直接打开文件
1. 在文件管理器中双击 `index.html` 文件
2. 浏览器将打开网站主页

### 方法二：使用本地服务器
如果你有 Python 安装，可以使用以下命令启动本地服务器：

```bash
# Python 3
python -m http.server 8000

# Python 2
python -m SimpleHTTPServer 8000
```

然后在浏览器中访问 `http://localhost:8000`

### 方法三：使用 Node.js
如果你有 Node.js 安装，可以使用：
```bash
npx serve .
```

## 功能测试

### 测试项目
1. **主页加载**：检查主页是否正常显示
2. **导航功能**：测试导航链接是否工作
3. **文档阅读器**：测试文档阅读器是否能加载和显示Markdown文件
4. **代码高亮**：检查代码块是否有语法高亮
5. **响应式设计**：在不同设备上测试网站显示效果

### 文档阅读器功能
1. 点击文档列表中的任意文档
2. 检查Markdown是否正确转换为HTML
3. 验证代码块的高亮显示
4. 测试文档间的链接跳转
5. 使用键盘左右箭头切换文档

## 故障排除

### 常见问题

#### 1. 文档无法加载
- 检查浏览器控制台是否有错误信息
- 确认 `docs/` 目录下的文件是否存在
- 检查文件路径是否正确

#### 2. 样式不生效
- 确认 `website/style.css` 文件是否存在
- 检查HTML文件中的CSS链接路径
- 清除浏览器缓存

#### 3. JavaScript错误
- 打开浏览器开发者工具查看控制台错误
- 检查 `website/script.js` 文件是否加载成功
- 确认外部CDN链接是否可用

#### 4. GitHub Pages部署问题
- 确认 `_config.yml` 文件配置正确
- 检查GitHub Pages设置是否启用
- 等待几分钟让部署生效

### 浏览器兼容性
网站已在以下浏览器测试：
- Chrome 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## 自定义配置

### 修改网站标题
编辑 `index.html` 和 `website/docs_reader.html` 中的 `<title>` 标签

### 修改颜色和样式
编辑 `website/style.css` 文件中的CSS变量和样式规则

### 添加新文档
1. 将Markdown文件放入 `docs/` 目录
2. 更新 `website/script.js` 中的 `docFiles` 数组
3. 重新部署网站

## 更新和维护

### 添加新功能
1. 在本地测试新功能
2. 提交更改到GitHub仓库
3. GitHub Pages会自动重新部署

### 更新文档
1. 直接编辑 `docs/` 目录下的Markdown文件
2. 提交更改后，网站会自动更新

## 联系支持
如遇到问题，请检查浏览器控制台错误信息，或在GitHub仓库中提交Issue。