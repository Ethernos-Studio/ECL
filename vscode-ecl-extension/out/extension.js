"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const vscode = require("vscode");
const path = require("path");
function activate(context) {
    console.log('ECL Language Support 插件已激活');
    // 注册代码补全提供者
    const eclCompletionProvider = vscode.languages.registerCompletionItemProvider('ecl', new ECLCompletionProvider(), '.', // 在输入点时触发
    ' ', // 在输入空格时触发
    '(' // 在输入括号时触发
    );
    // 注册悬停提示提供者
    const eclHoverProvider = vscode.languages.registerHoverProvider('ecl', new ECLHoverProvider());
    // 注册文档符号提供者
    const eclDocumentSymbolProvider = vscode.languages.registerDocumentSymbolProvider('ecl', new ECLDocumentSymbolProvider());
    // 注册定义提供者（支持Ctrl+点击跳转）
    const eclDefinitionProvider = vscode.languages.registerDefinitionProvider('ecl', new ECLDefinitionProvider());
    // 注册引用提供者（支持查找所有引用）
    const eclReferenceProvider = vscode.languages.registerReferenceProvider('ecl', new ECLReferenceProvider());
    context.subscriptions.push(eclCompletionProvider, eclHoverProvider, eclDocumentSymbolProvider, eclDefinitionProvider, eclReferenceProvider);
}
exports.activate = activate;
function deactivate() {
    console.log('ECL Language Support 插件已停用');
}
exports.deactivate = deactivate;
// 代码补全提供者
class ECLCompletionProvider {
    provideCompletionItems(document, position, token, context) {
        const completions = [];
        const linePrefix = document.lineAt(position).text.substr(0, position.character);
        // 关键字补全
        const keywords = [
            'var', 'func', 'expr', 'if', 'else', 'for', 'while', 'in',
            'return', 'print', 'println', 'input', 'import', 'true', 'false'
        ];
        keywords.forEach(keyword => {
            const item = new vscode.CompletionItem(keyword, vscode.CompletionItemKind.Keyword);
            item.detail = 'ECL 关键字';
            completions.push(item);
        });
        // 类型补全
        const types = ['int', 'str', 'bool', 'float', 'double'];
        types.forEach(type => {
            const item = new vscode.CompletionItem(type, vscode.CompletionItemKind.Class);
            item.detail = 'ECL 数据类型';
            completions.push(item);
        });
        // 内置函数补全
        const builtinFunctions = [
            { name: 'print', detail: '打印到控制台' },
            { name: 'println', detail: '打印并换行' },
            { name: 'input', detail: '获取用户输入' }
        ];
        builtinFunctions.forEach(func => {
            const item = new vscode.CompletionItem(func.name, vscode.CompletionItemKind.Function);
            item.detail = func.detail;
            completions.push(item);
        });
        // 智能补全：如果在输入 < 后，提供类型补全
        if (linePrefix.endsWith('<')) {
            const typeCompletions = types.map(type => {
                const item = new vscode.CompletionItem(`${type}>`, vscode.CompletionItemKind.Class);
                item.detail = '类型声明';
                return item;
            });
            return typeCompletions;
        }
        // 智能补全：如果在输入 . 后，检查是否是范围运算符
        if (linePrefix.endsWith('.')) {
            const rangeItem = new vscode.CompletionItem('.', vscode.CompletionItemKind.Operator);
            rangeItem.detail = '范围运算符 (..)';
            rangeItem.insertText = '.';
            completions.push(rangeItem);
        }
        return completions;
    }
}
// 悬停提示提供者
class ECLHoverProvider {
    provideHover(document, position, token) {
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) {
            return;
        }
        const word = document.getText(wordRange);
        // 关键字悬停提示
        const keywordDocs = {
            'var': '声明变量：var <type>name = value 或 var name = value',
            'func': '声明函数：func name(parameters) { body }',
            'expr': '声明表达式函数：expr name(parameters) { body }',
            'if': '条件语句：if (condition) { body }',
            'else': '条件语句的else分支：} else { body }',
            'for': '循环语句：for i in range { body }',
            'while': '循环语句：while (condition) { body }',
            'print': '打印函数：print(message)',
            'println': '打印函数：println(message)',
            'input': '输入函数：input "prompt", variable',
            'import': '导入文件：import "filename.ecl"',
            'return': '返回语句：return value',
            'int': '整数类型',
            'str': '字符串类型',
            'bool': '布尔类型',
            'float': '浮点数类型',
            'double': '双精度浮点数类型'
        };
        if (keywordDocs[word]) {
            return new vscode.Hover(new vscode.MarkdownString(keywordDocs[word]));
        }
        return;
    }
}
// 文档符号提供者
class ECLDocumentSymbolProvider {
    provideDocumentSymbols(document, token) {
        const symbols = [];
        const text = document.getText();
        const lines = text.split('\n');
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i].trim();
            // 匹配函数定义
            const funcMatch = line.match(/^\s*(?:func|expr)\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/);
            if (funcMatch) {
                const funcName = funcMatch[1];
                const range = new vscode.Range(i, 0, i, line.length);
                const symbol = new vscode.DocumentSymbol(funcName, '函数', vscode.SymbolKind.Function, range, range);
                symbols.push(symbol);
            }
            // 匹配变量定义
            const varMatch = line.match(/^\s*var\s+(?:<[^>]+>)?([a-zA-Z_][a-zA-Z0-9_]*)/);
            if (varMatch) {
                const varName = varMatch[1];
                const range = new vscode.Range(i, 0, i, line.length);
                const symbol = new vscode.DocumentSymbol(varName, '变量', vscode.SymbolKind.Variable, range, range);
                symbols.push(symbol);
            }
        }
        return symbols;
    }
}
// 定义提供者（支持Ctrl+点击跳转，包括跨文件import）
class ECLDefinitionProvider {
    provideDefinition(document, position, token) {
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) {
            return;
        }
        const word = document.getText(wordRange);
        // 首先在当前文件中查找
        const currentFileResult = this.findDefinitionInDocument(document, word);
        if (currentFileResult) {
            return currentFileResult;
        }
        // 如果在当前文件中没有找到，查找import的文件
        return this.findImportedSymbols(document, word);
    }
    findDefinitionInDocument(document, word) {
        const text = document.getText();
        const lines = text.split('\n');
        // 查找函数定义
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            // 匹配函数定义：func name( 或 expr name(
            const funcMatch = line.match(new RegExp(`\\b(?:func|expr)\\s+(${word})\\s*\\(`));
            if (funcMatch) {
                const funcNameIndex = line.indexOf(word);
                return new vscode.Location(document.uri, new vscode.Range(i, funcNameIndex, i, funcNameIndex + word.length));
            }
            // 匹配变量定义：var name 或 var <type>name
            const varMatch = line.match(new RegExp(`\\bvar\\s+(?:<[^>]+>)?(${word})\\b`));
            if (varMatch) {
                const varNameIndex = line.indexOf(word);
                return new vscode.Location(document.uri, new vscode.Range(i, varNameIndex, i, varNameIndex + word.length));
            }
        }
        return undefined;
    }
    findImportedSymbols(document, word) {
        const text = document.getText();
        const lines = text.split('\n');
        const importStatements = this.extractImportStatements(lines);
        // 获取当前文件所在目录和工作区根目录
        const currentFileDir = path.dirname(document.uri.fsPath);
        const workspaceRoot = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath || currentFileDir;
        // 为每个import语句创建查找promise
        const searchPromises = importStatements.map(importFile => {
            const possiblePaths = [
                path.join(currentFileDir, importFile),
                path.join(currentFileDir, importFile + '.ecl'),
                path.join(workspaceRoot, importFile),
                path.join(workspaceRoot, importFile + '.ecl')
            ];
            // 为每个可能的路径创建查找promise
            const pathPromises = possiblePaths.map(filePath => {
                const uri = vscode.Uri.file(filePath);
                return vscode.workspace.openTextDocument(uri).then(importedDocument => {
                    const definition = this.findDefinitionInDocument(importedDocument, word);
                    return definition;
                }).then((result) => {
                    return result; // 返回找到的定义或undefined
                }, () => {
                    return undefined; // 文件不存在，返回undefined
                });
            });
            // 返回第一个找到的定义
            return Promise.all(pathPromises).then((results) => {
                return results.find((result) => result !== undefined);
            });
        });
        // 返回所有import语句的查找结果中的第一个有效定义
        return Promise.all(searchPromises).then((allResults) => {
            return allResults.find((result) => result !== undefined);
        });
    }
    extractImportStatements(lines) {
        const imports = [];
        const importRegex = /import\s+["']([^"']+)["']/g;
        for (const line of lines) {
            const matches = line.matchAll(importRegex);
            for (const match of matches) {
                let importPath = match[1];
                // 移除.ecl扩展名如果存在
                if (importPath.endsWith('.ecl')) {
                    importPath = importPath.slice(0, -4);
                }
                imports.push(importPath);
            }
        }
        return imports;
    }
}
// 引用提供者（支持查找所有引用）
class ECLReferenceProvider {
    provideReferences(document, position, context, token) {
        const wordRange = document.getWordRangeAtPosition(position);
        if (!wordRange) {
            return;
        }
        const word = document.getText(wordRange);
        const text = document.getText();
        const lines = text.split('\n');
        const references = [];
        // 查找所有引用
        for (let i = 0; i < lines.length; i++) {
            const line = lines[i];
            // 跳过定义行（除非包含声明）
            if (line.includes(`func ${word}`) || line.includes(`expr ${word}`) ||
                line.match(new RegExp(`\\bvar\\s+(?:<[^>]+>)?${word}\\b`))) {
                if (!context.includeDeclaration) {
                    continue;
                }
            }
            // 查找变量或函数引用
            const wordRegex = new RegExp(`\\b${word}\\b`, 'g');
            let match;
            while ((match = wordRegex.exec(line)) !== null) {
                references.push(new vscode.Location(document.uri, new vscode.Range(i, match.index, i, match.index + word.length)));
            }
        }
        return references;
    }
}
//# sourceMappingURL=extension.js.map