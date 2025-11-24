// 全局变量
let currentFile = null;
let docFiles = [];

// 初始化函数
document.addEventListener('DOMContentLoaded', function() {
    // 检查当前页面
    if (document.getElementById('doc-list')) {
        // 文档阅读器页面
        loadDocumentList();
        loadDocumentFromURL();
    }
});

// 加载文档列表
async function loadDocumentList() {
    try {
        // 获取docs目录下的所有markdown文件
        const response = await fetch('../docs/');
        if (!response.ok) {
            throw new Error('无法加载文档列表');
        }
        
        // 由于GitHub Pages不支持目录列表，我们需要手动定义文档列表
        docFiles = [
            'introduction.md',
            'quickstart.md',
            'installation.md',
            'syntax.md',
            'types.md',
            'variables.md',
            'functions.md',
            'control-flow.md',
            'data-structures.md',
            'io.md',
            'expr-functions.md',
            'string-escapes.md',
            'type-conversion.md',
            'debugging.md',
            'repl.md',
            'examples.md',
            'README.md'
        ];

        displayDocumentList(docFiles);
    } catch (error) {
        console.error('加载文档列表失败:', error);
        showError('加载文档列表失败: ' + error.message);
    }
}

// 显示文档列表
function displayDocumentList(files) {
    const docList = document.getElementById('doc-list');
    if (!docList) return;

    docList.innerHTML = '';
    
    files.forEach(file => {
        const listItem = document.createElement('div');
        listItem.className = 'doc-list-item';
        
        const link = document.createElement('a');
        link.href = '#';
        link.textContent = formatFileName(file);
        link.onclick = (e) => {
            e.preventDefault();
            loadDocument(file);
        };
        
        listItem.appendChild(link);
        docList.appendChild(listItem);
    });
}

// 格式化文件名
function formatFileName(filename) {
    // 移除.md扩展名并替换连字符为空格
    const name = filename.replace('.md', '').replace(/-/g, ' ');
    // 将每个单词的首字母大写
    return name.split(' ').map(word => 
        word.charAt(0).toUpperCase() + word.slice(1)
    ).join(' ');
}

// 从URL参数加载文档
function loadDocumentFromURL() {
    const urlParams = new URLSearchParams(window.location.search);
    const file = urlParams.get('file');
    
    if (file) {
        loadDocument(file);
    } else {
        // 默认加载第一个文档
        if (docFiles.length > 0) {
            loadDocument(docFiles[0]);
        }
    }
}

// 加载指定文档
async function loadDocument(filename) {
    const loading = document.getElementById('loading');
    const error = document.getElementById('error');
    const content = document.getElementById('markdown-content');
    
    // 显示加载状态
    loading.style.display = 'block';
    error.style.display = 'none';
    content.innerHTML = '';
    
    currentFile = filename;
    
    try {
        // 构建正确的文件路径
        const filePath = `../docs/${filename}`;
        const response = await fetch(filePath);
        
        if (!response.ok) {
            throw new Error(`无法加载文档: ${filename}`);
        }
        
        const markdownText = await response.text();
        const htmlContent = convertMarkdownToHtml(markdownText);
        
        // 显示转换后的HTML
        content.innerHTML = htmlContent;
        
        // 高亮代码块
        highlightCodeBlocks();
        
        // 更新活跃的文档列表项
        updateActiveDocumentItem(filename);
        
        // 更新URL
        updateURL(filename);
        
    } catch (error) {
        console.error('加载文档失败:', error);
        showError('加载文档失败: ' + error.message);
    } finally {
        loading.style.display = 'none';
    }
}

// 将Markdown转换为HTML
function convertMarkdownToHtml(markdown) {
    // 配置marked选项
    marked.setOptions({
        highlight: function(code, lang) {
            if (Prism.languages[lang]) {
                return Prism.highlight(code, Prism.languages[lang], lang);
            }
            return code;
        },
        langPrefix: 'language-',
        breaks: true,
        gfm: true
    });

    // 自定义渲染器
    const renderer = new marked.Renderer();
    
    // 自定义链接渲染，确保相对链接正确处理
    renderer.link = function(href, title, text) {
        // 处理相对链接
        if (href.startsWith('./') || href.startsWith('../')) {
            // 保持原始链接，让浏览器处理
            return `<a href="${href}" title="${title || ''}" target="_blank">${text}</a>`;
        }
        
        // 处理文档间的链接
        if (href.endsWith('.md')) {
            const linkFile = href.split('/').pop();
            return `<a href="#" onclick="loadDocument('${linkFile}'); return false;" title="${title || ''}">${text}</a>`;
        }
        
        // 其他链接
        const titleAttr = title ? ` title="${title}"` : '';
        return `<a href="${href}"${titleAttr} target="_blank">${text}</a>`;
    };

    // 自定义代码块渲染
    renderer.code = function(code, language) {
        const validLang = language && Prism.languages[language] ? language : 'text';
        const highlighted = Prism.highlight(code, Prism.languages[validLang], validLang);
        return `<pre><code class="language-${validLang}">${highlighted}</code></pre>`;
    };

    // 转换Markdown为HTML
    return marked.parse(markdown, { renderer });
}

// 高亮代码块
function highlightCodeBlocks() {
    // 使用Prism.js高亮代码
    if (typeof Prism !== 'undefined') {
        Prism.highlightAll();
    }
}

// 更新活跃的文档列表项
function updateActiveDocumentItem(filename) {
    const docItems = document.querySelectorAll('.doc-list-item');
    docItems.forEach(item => {
        item.classList.remove('active');
        const link = item.querySelector('a');
        if (link && link.textContent === formatFileName(filename)) {
            item.classList.add('active');
        }
    });
}

// 更新URL
function updateURL(filename) {
    const url = new URL(window.location);
    url.searchParams.set('file', filename);
    window.history.pushState({}, '', url);
}

// 显示错误信息
function showError(message) {
    const error = document.getElementById('error');
    const content = document.getElementById('markdown-content');
    
    error.textContent = message;
    error.style.display = 'block';
    content.innerHTML = '';
}

// 搜索功能（可选）
function searchDocuments(query) {
    if (!query.trim()) {
        displayDocumentList(docFiles);
        return;
    }
    
    const filtered = docFiles.filter(file => 
        file.toLowerCase().includes(query.toLowerCase()) ||
        formatFileName(file).toLowerCase().includes(query.toLowerCase())
    );
    
    displayDocumentList(filtered);
}

// 键盘快捷键
document.addEventListener('keydown', function(e) {
    // Ctrl/Cmd + K 显示搜索（如果有搜索框）
    if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        // 这里可以添加搜索功能
    }
    
    // 左右箭头键切换文档
    if (e.key === 'ArrowLeft' || e.key === 'ArrowRight') {
        const currentIndex = docFiles.indexOf(currentFile);
        if (currentIndex !== -1) {
            let newIndex;
            if (e.key === 'ArrowLeft') {
                newIndex = currentIndex > 0 ? currentIndex - 1 : docFiles.length - 1;
            } else {
                newIndex = currentIndex < docFiles.length - 1 ? currentIndex + 1 : 0;
            }
            loadDocument(docFiles[newIndex]);
        }
    }
});

// 处理浏览器后退/前进按钮
window.addEventListener('popstate', function() {
    loadDocumentFromURL();
});

// 工具函数：防抖
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

// 工具函数：节流
function throttle(func, limit) {
    let inThrottle;
    return function() {
        const args = arguments;
        const context = this;
        if (!inThrottle) {
            func.apply(context, args);
            inThrottle = true;
            setTimeout(() => inThrottle = false, limit);
        }
    };
}

// 导出函数供全局使用（如果需要）
window.loadDocument = loadDocument;
window.searchDocuments = searchDocuments;