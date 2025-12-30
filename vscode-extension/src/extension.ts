import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient;

// =============================================================================
// SEMANTIC TOKEN TYPES - Sanskrit Linguistics Mapping
// =============================================================================

const tokenTypes = [
  "namespace", // 0: Modules, imports
  "type", // 1: Types, structs
  "class", // 2: Divine weapons (Astras)
  "enum", // 3: Enums, Narakas
  "interface", // 4: Traits
  "struct", // 5: Structs
  "typeParameter", // 6: Generics
  "parameter", // 7: Function params
  "variable", // 8: Variables
  "property", // 9: Fields
  "enumMember", // 10: Enum variants
  "event", // 11: Events
  "function", // 12: Functions
  "method", // 13: Methods
  "macro", // 14: Macros
  "keyword", // 15: Keywords
  "modifier", // 16: Affixes, modifiers
  "comment", // 17: Comments
  "string", // 18: Strings
  "number", // 19: Numbers
  "regexp", // 20: Regex
  "operator", // 21: Operators
  // Custom Sanskrit tokens
  "karaka", // 22: Semantic roles [kartṛ], [karman]
  "astra", // 23: Divine weapons (optimizations)
  "naraka", // 24: Hell types (errors)
  "guna", // 25: Qualities (sattva/rajas/tamas)
  "kosha", // 26: Sheaths (memory regions)
  "chakra", // 27: Energy centers (layers)
  "varna", // 28: Privilege rings
  "marga", // 29: Paths (optimization strategies)
  "mantra", // 30: Sacred invocations
  "lifetime", // 31: Lifetime annotations
];

const tokenModifiers = [
  "declaration",
  "definition",
  "readonly",
  "static",
  "deprecated",
  "abstract",
  "async",
  "modification",
  "documentation",
  "defaultLibrary",
  // Custom Sanskrit modifiers
  "mutable", // -ā affix
  "immutable", // -a affix
  "linear", // -l affix
  "borrowed", // -b affix
  "global", // -g affix
  "threadsafe", // -sūtra affix
  "secure", // -vaitaraṇī affix
  "sattva", // Pure/hint
  "rajas", // Active/warning
  "tamas", // Dark/error
];

const legend = new vscode.SemanticTokensLegend(tokenTypes, tokenModifiers);

// =============================================================================
// SEMANTIC TOKENS PROVIDER - Intelligent Syntax Analysis
// =============================================================================

class JagannathSemanticTokensProvider
  implements vscode.DocumentSemanticTokensProvider
{
  provideDocumentSemanticTokens(
    document: vscode.TextDocument
  ): vscode.ProviderResult<vscode.SemanticTokens> {
    const builder = new vscode.SemanticTokensBuilder(legend);
    const text = document.getText();
    const lines = text.split("\n");

    for (let lineNum = 0; lineNum < lines.length; lineNum++) {
      const line = lines[lineNum];
      this.tokenizeLine(builder, line, lineNum);
    }

    return builder.build();
  }

  private tokenizeLine(
    builder: vscode.SemanticTokensBuilder,
    line: string,
    lineNum: number
  ) {
    // Divine Weapons (Astras) - Optimization passes
    this.matchPattern(
      builder,
      line,
      lineNum,
      /\b(Brahmastra|Pashupatastra|Agneyastra|Varunastra|SudarshanaChakra|Narayanastra|Vayavyastra|Nagastra|Garudastra|Mohiniastra|Vajra|Trishula|Gandiva|Vijaya)\b/g,
      "astra",
      ["definition"]
    );

    // Narakas - Error types from Garuda Purana
    this.matchPattern(
      builder,
      line,
      lineNum,
      /\b(Tamisram|Andhakupa|Raurava|Suchimukha|Vaitarani|Kalasutra|Asipatravana|Kumbhipaka|Preta|Pranarodha|Visasana)\b/g,
      "naraka",
      ["deprecated"]
    );

    // Guṇas - Quality levels
    this.matchPattern(
      builder,
      line,
      lineNum,
      /\b(sattva|rajas|tamas|sāttvika|rājasika|tāmasika)\b/g,
      "guna",
      []
    );

    // Kośas - Memory sheaths
    this.matchPattern(
      builder,
      line,
      lineNum,
      /\b(anna(maya)?|prāṇa(maya)?|mano(maya)?|vijñāna(maya)?|ānanda(maya)?)\b/gi,
      "kosha",
      []
    );

    // Chakras - Software layers
    this.matchPattern(
      builder,
      line,
      lineNum,
      /\b(mūlādhāra|muladhara|svādhiṣṭhāna|svadhisthana|maṇipūra|manipura|anāhata|anahata|viśuddha|vishuddha|ājñā|ajna|sahasrāra|sahasrara)\b/g,
      "chakra",
      []
    );

    // Varṇas - Privilege rings
    this.matchPattern(
      builder,
      line,
      lineNum,
      /\b(brāhmaṇa|brahmana|brahmin|kṣatriya|kshatriya|vaiśya|vaishya|śūdra|shudra)\b/g,
      "varna",
      []
    );

    // Mārgas - Optimization paths
    this.matchPattern(
      builder,
      line,
      lineNum,
      /\b(karma(yoga)?|jñāna(yoga)?|jnana(yoga)?|bhakti(yoga)?|rāja(yoga)?|raja(yoga)?)\b/g,
      "marga",
      []
    );

    // Kārakas - Semantic roles
    this.matchPattern(
      builder,
      line,
      lineNum,
      /\[(kartṛ|kartr|karman|karaṇa|karana|sampradāna|sampradana|apādāna|apadana|adhikaraṇa|adhikarana|sambandha|hetu)\]/g,
      "karaka",
      []
    );

    // Mantras - Sacred invocations
    this.matchPattern(
      builder,
      line,
      lineNum,
      /\b(oṁ|om|namaḥ|svāhā|phaṭ|huṁ)\b/g,
      "mantra",
      []
    );

    // Affixes - Type modifiers
    this.matchPattern(
      builder,
      line,
      lineNum,
      /-[aāblgkhpsr](?=[-\s\{\(]|$)/g,
      "modifier",
      ["declaration"]
    );

    // Thread-safety and security affixes
    this.matchPattern(
      builder,
      line,
      lineNum,
      /-(sūtra|sutra|vaitaraṇī|vaitarani)\b/g,
      "modifier",
      ["threadsafe"]
    );

    // Lifetime annotations
    this.matchPattern(
      builder,
      line,
      lineNum,
      /\^[0-9]+|'[a-z][a-z0-9_]*/g,
      "lifetime",
      []
    );

    // Macros
    this.matchPattern(
      builder,
      line,
      lineNum,
      /\b(mudraṇa|mudrana|oṁ|om|parikṣā|pariksha|debug|assert|panic|todo|unreachable)!/g,
      "macro",
      []
    );
  }

  private matchPattern(
    builder: vscode.SemanticTokensBuilder,
    line: string,
    lineNum: number,
    pattern: RegExp,
    tokenType: string,
    modifiers: string[]
  ) {
    let match: RegExpExecArray | null;
    while ((match = pattern.exec(line)) !== null) {
      const typeIndex = tokenTypes.indexOf(tokenType);
      if (typeIndex >= 0) {
        const modifierBits = modifiers.reduce((acc, mod) => {
          const idx = tokenModifiers.indexOf(mod);
          return idx >= 0 ? acc | (1 << idx) : acc;
        }, 0);
        builder.push(
          lineNum,
          match.index,
          match[0].length,
          typeIndex,
          modifierBits
        );
      }
    }
  }
}

// =============================================================================
// CODE LENS PROVIDER - Philosophical Annotations
// =============================================================================

class JagannathCodeLensProvider implements vscode.CodeLensProvider {
  provideCodeLenses(document: vscode.TextDocument): vscode.CodeLens[] {
    const codeLenses: vscode.CodeLens[] = [];
    const text = document.getText();
    const lines = text.split("\n");

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];

      // Function definitions
      const fnMatch = line.match(/kāryakrama\s+([a-zA-Zā-ṛ_][a-zA-Zā-ṛ0-9_]*)/);
      if (fnMatch) {
        const range = new vscode.Range(i, 0, i, line.length);
        codeLenses.push(
          new vscode.CodeLens(range, {
            title: "🕉️ Analyze Function",
            command: "jagannath.analyzeFunction",
            arguments: [fnMatch[1], document.uri, i],
          })
        );
      }

      // Struct definitions
      const structMatch = line.match(/prakāra\s+([A-ZĀ-Ṛ][a-zA-Zā-ṛ0-9_]*)/);
      if (structMatch) {
        const range = new vscode.Range(i, 0, i, line.length);
        codeLenses.push(
          new vscode.CodeLens(range, {
            title: "📊 View Memory Layout",
            command: "jagannath.viewMemoryLayout",
            arguments: [structMatch[1], document.uri, i],
          })
        );
      }

      // Astra references
      if (/Brahmastra|Agneyastra|Varunastra|Pashupatastra/.test(line)) {
        const range = new vscode.Range(i, 0, i, line.length);
        codeLenses.push(
          new vscode.CodeLens(range, {
            title: "⚔️ View Optimization",
            command: "jagannath.viewAstraOptimization",
            arguments: [document.uri, i],
          })
        );
      }

      // Naraka error patterns
      if (/Naraka|Tamisram|Vaitarani|Suchimukha/.test(line)) {
        const range = new vscode.Range(i, 0, i, line.length);
        codeLenses.push(
          new vscode.CodeLens(range, {
            title: "🔥 View Error Classification",
            command: "jagannath.viewNarakaClassification",
            arguments: [document.uri, i],
          })
        );
      }
    }

    return codeLenses;
  }
}

// =============================================================================
// HOVER PROVIDER - Sanskrit Linguistic Information
// =============================================================================

class JagannathHoverProvider implements vscode.HoverProvider {
  private readonly glossary: Map<
    string,
    { sanskrit: string; meaning: string; context: string }
  > = new Map([
    // Keywords
    [
      "kāryakrama",
      {
        sanskrit: "कार्यक्रम",
        meaning: "function/procedure",
        context: "From kārya (task) + krama (sequence)",
      },
    ],
    [
      "māna",
      {
        sanskrit: "मान",
        meaning: "variable declaration",
        context: "From √man (to think/measure)",
      },
    ],
    [
      "yad",
      {
        sanskrit: "यद्",
        meaning: "if/when conditional",
        context: "Relative pronoun 'when'",
      },
    ],
    [
      "anyathā",
      {
        sanskrit: "अन्यथा",
        meaning: "else/otherwise",
        context: "anya (other) + thā (manner)",
      },
    ],
    [
      "cala",
      {
        sanskrit: "चल",
        meaning: "loop/iteration",
        context: "From √cal (to move)",
      },
    ],
    [
      "phera",
      {
        sanskrit: "फेर",
        meaning: "return",
        context: "From √phr (to return/bring back)",
      },
    ],

    // Types
    [
      "saṅkhyā",
      {
        sanskrit: "संख्या",
        meaning: "number",
        context: "From √saṅkhyā (to count)",
      },
    ],
    [
      "sūtra",
      {
        sanskrit: "सूत्र",
        meaning: "string/thread",
        context: "From √siv (to sew)",
      },
    ],
    [
      "tarka",
      {
        sanskrit: "तर्क",
        meaning: "boolean/logic",
        context: "From √tark (to reason)",
      },
    ],

    // Philosophy
    [
      "sattva",
      {
        sanskrit: "सत्त्व",
        meaning: "purity/goodness",
        context: "Guṇa: Pure quality - hints/info",
      },
    ],
    [
      "rajas",
      {
        sanskrit: "रजस्",
        meaning: "passion/activity",
        context: "Guṇa: Active quality - warnings",
      },
    ],
    [
      "tamas",
      {
        sanskrit: "तमस्",
        meaning: "darkness/inertia",
        context: "Guṇa: Dark quality - errors",
      },
    ],

    // Astras
    [
      "Brahmastra",
      {
        sanskrit: "ब्रह्मास्त्र",
        meaning: "Ultimate weapon",
        context: "Dead code elimination - destroys all unreachable code",
      },
    ],
    [
      "Agneyastra",
      {
        sanskrit: "आग्नेयास्त्र",
        meaning: "Fire weapon",
        context: "CPU-intensive optimization - burns away inefficiency",
      },
    ],
    [
      "Varunastra",
      {
        sanskrit: "वारुणास्त्र",
        meaning: "Water weapon",
        context: "Memory flow analysis - controls data streams",
      },
    ],
    [
      "Pashupatastra",
      {
        sanskrit: "पाशुपतास्त्र",
        meaning: "Shiva's weapon",
        context: "Destructive refactoring - transforms entire structures",
      },
    ],

    // Narakas
    [
      "Tamisram",
      {
        sanskrit: "तामिस्रम्",
        meaning: "Hell of darkness",
        context: "Use-after-free errors - memory stolen like theft",
      },
    ],
    [
      "Suchimukha",
      {
        sanskrit: "सूचीमुख",
        meaning: "Needle-face hell",
        context: "Memory leaks - objects trapped, never freed",
      },
    ],
    [
      "Vaitarani",
      {
        sanskrit: "वैतरणी",
        meaning: "Filthy river hell",
        context: "Tainted data crossing security boundary",
      },
    ],
    [
      "Andhakupa",
      {
        sanskrit: "अन्धकूप",
        meaning: "Blind well hell",
        context: "Null pointer dereference - falling into void",
      },
    ],

    // Kośas
    [
      "annamaya",
      {
        sanskrit: "अन्नमय",
        meaning: "Food sheath",
        context: "Register allocation - fastest, most physical",
      },
    ],
    [
      "prāṇamaya",
      {
        sanskrit: "प्राणमय",
        meaning: "Vital sheath",
        context: "Stack allocation - automatic, short-lived",
      },
    ],
    [
      "manomaya",
      {
        sanskrit: "मनोमय",
        meaning: "Mind sheath",
        context: "Heap allocation - flexible, managed",
      },
    ],
    [
      "vijñānamaya",
      {
        sanskrit: "विज्ञानमय",
        meaning: "Wisdom sheath",
        context: "Global/static allocation - program-wide",
      },
    ],
    [
      "ānandamaya",
      {
        sanskrit: "आनन्दमय",
        meaning: "Bliss sheath",
        context: "Eternal constants - compile-time values",
      },
    ],

    // Kārakas
    [
      "kartṛ",
      {
        sanskrit: "कर्तृ",
        meaning: "Agent/doer",
        context: "Subject performing the action",
      },
    ],
    [
      "karman",
      {
        sanskrit: "कर्मन्",
        meaning: "Patient/object",
        context: "Object receiving the action",
      },
    ],
    [
      "karaṇa",
      {
        sanskrit: "करण",
        meaning: "Instrument",
        context: "Means by which action is performed",
      },
    ],

    // Affixes
    [
      "-ā",
      {
        sanskrit: "आ",
        meaning: "Mutable marker",
        context: "Variable can be modified after initialization",
      },
    ],
    [
      "-a",
      {
        sanskrit: "अ",
        meaning: "Immutable marker",
        context: "Variable cannot be changed (default)",
      },
    ],
    [
      "-l",
      {
        sanskrit: "ल्",
        meaning: "Linear ownership",
        context: "Single owner, must be consumed or dropped",
      },
    ],
    [
      "-b",
      {
        sanskrit: "ब्",
        meaning: "Borrowed reference",
        context: "Temporary access, original owner retains",
      },
    ],
    [
      "-k",
      {
        sanskrit: "क्",
        meaning: "Stack allocation",
        context: "Allocated on call stack, auto-freed",
      },
    ],
    [
      "-h",
      {
        sanskrit: "ह्",
        meaning: "Heap allocation",
        context: "Allocated on heap, explicit management",
      },
    ],
    [
      "-sūtra",
      {
        sanskrit: "सूत्र",
        meaning: "Thread-safe",
        context: "Can be safely shared across threads",
      },
    ],
  ]);

  provideHover(
    document: vscode.TextDocument,
    position: vscode.Position
  ): vscode.ProviderResult<vscode.Hover> {
    const range = document.getWordRangeAtPosition(
      position,
      /[a-zA-Zā-ṛĀ-Ṛ_][a-zA-Zā-ṛĀ-Ṛ0-9_-]*/
    );
    if (!range) return null;

    const word = document.getText(range);

    // Check glossary
    const entry =
      this.glossary.get(word) || this.glossary.get(word.toLowerCase());
    if (entry) {
      const markdown = new vscode.MarkdownString();
      markdown.appendMarkdown(`## ${entry.sanskrit} (${word})\n\n`);
      markdown.appendMarkdown(`**Meaning:** ${entry.meaning}\n\n`);
      markdown.appendMarkdown(`*${entry.context}*`);
      markdown.isTrusted = true;
      return new vscode.Hover(markdown, range);
    }

    // Check for affixes
    const affixEntry = this.glossary.get("-" + word);
    if (affixEntry) {
      const markdown = new vscode.MarkdownString();
      markdown.appendMarkdown(`## Affix: ${affixEntry.sanskrit}\n\n`);
      markdown.appendMarkdown(`**Meaning:** ${affixEntry.meaning}\n\n`);
      markdown.appendMarkdown(`*${affixEntry.context}*`);
      return new vscode.Hover(markdown, range);
    }

    return null;
  }
}

// =============================================================================
// WEBVIEW PANELS - Graphical Tools
// =============================================================================

class ASTViewerPanel {
  public static currentPanel: ASTViewerPanel | undefined;
  private readonly _panel: vscode.WebviewPanel;
  private _disposables: vscode.Disposable[] = [];

  private constructor(panel: vscode.WebviewPanel, extensionUri: vscode.Uri) {
    this._panel = panel;
    this._panel.onDidDispose(() => this.dispose(), null, this._disposables);
    this._panel.webview.html = this._getHtmlContent();
  }

  public static createOrShow(extensionUri: vscode.Uri) {
    const column = vscode.window.activeTextEditor
      ? vscode.window.activeTextEditor.viewColumn
      : undefined;

    if (ASTViewerPanel.currentPanel) {
      ASTViewerPanel.currentPanel._panel.reveal(column);
      return;
    }

    const panel = vscode.window.createWebviewPanel(
      "jagannathAstViewer",
      "🕉️ AST Viewer - Jagannath",
      column || vscode.ViewColumn.Two,
      {
        enableScripts: true,
        retainContextWhenHidden: true,
      }
    );

    ASTViewerPanel.currentPanel = new ASTViewerPanel(panel, extensionUri);
  }

  public updateAST(ast: any) {
    this._panel.webview.postMessage({ type: "updateAST", ast });
  }

  private _getHtmlContent(): string {
    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Jagannath AST Viewer</title>
  <style>
    :root {
      --bg-primary: #0d1117;
      --bg-secondary: #161b22;
      --bg-tertiary: #21262d;
      --text-primary: #c9d1d9;
      --text-secondary: #8b949e;
      --accent-sattva: #7ee787;    /* Green - purity */
      --accent-rajas: #ffa657;     /* Orange - activity */
      --accent-tamas: #f85149;     /* Red - darkness */
      --accent-brahman: #a371f7;   /* Purple - divine */
      --accent-vishnu: #58a6ff;    /* Blue - preserver */
      --accent-shiva: #ff7b72;     /* Coral - destroyer */
      --border-color: #30363d;
    }

    body {
      font-family: 'Segoe UI', system-ui, sans-serif;
      background: var(--bg-primary);
      color: var(--text-primary);
      margin: 0;
      padding: 20px;
    }

    .header {
      display: flex;
      align-items: center;
      gap: 12px;
      margin-bottom: 24px;
      padding-bottom: 16px;
      border-bottom: 1px solid var(--border-color);
    }

    .header h1 {
      font-size: 1.5rem;
      margin: 0;
      background: linear-gradient(135deg, var(--accent-brahman), var(--accent-vishnu));
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }

    .om-symbol {
      font-size: 2rem;
      color: var(--accent-brahman);
    }

    .toolbar {
      display: flex;
      gap: 8px;
      margin-bottom: 16px;
    }

    .toolbar button {
      background: var(--bg-secondary);
      border: 1px solid var(--border-color);
      color: var(--text-primary);
      padding: 8px 16px;
      border-radius: 6px;
      cursor: pointer;
      font-size: 0.875rem;
      transition: all 0.2s;
    }

    .toolbar button:hover {
      background: var(--bg-tertiary);
      border-color: var(--accent-vishnu);
    }

    .toolbar button.active {
      background: var(--accent-vishnu);
      color: white;
    }

    .ast-container {
      background: var(--bg-secondary);
      border: 1px solid var(--border-color);
      border-radius: 8px;
      padding: 16px;
      overflow: auto;
      max-height: calc(100vh - 200px);
    }

    .tree-node {
      margin-left: 20px;
      position: relative;
    }

    .tree-node::before {
      content: '';
      position: absolute;
      left: -15px;
      top: 0;
      height: 100%;
      width: 1px;
      background: var(--border-color);
    }

    .node-header {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 6px 10px;
      margin: 2px 0;
      border-radius: 4px;
      cursor: pointer;
      transition: background 0.2s;
    }

    .node-header:hover {
      background: var(--bg-tertiary);
    }

    .node-type {
      font-weight: 600;
      color: var(--accent-vishnu);
    }

    .node-type.function { color: var(--accent-brahman); }
    .node-type.type { color: var(--accent-sattva); }
    .node-type.expression { color: var(--accent-rajas); }
    .node-type.statement { color: var(--accent-vishnu); }
    .node-type.error { color: var(--accent-tamas); }

    .node-name {
      color: var(--text-secondary);
      font-style: italic;
    }

    .node-badge {
      font-size: 0.7rem;
      padding: 2px 6px;
      border-radius: 10px;
      background: var(--bg-tertiary);
      color: var(--text-secondary);
    }

    .toggle-icon {
      width: 16px;
      height: 16px;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 0.75rem;
    }

    .collapsed > .tree-children {
      display: none;
    }

    .empty-state {
      text-align: center;
      padding: 60px 20px;
      color: var(--text-secondary);
    }

    .empty-state .om {
      font-size: 4rem;
      color: var(--accent-brahman);
      opacity: 0.5;
      margin-bottom: 16px;
    }

    .philosophy-panel {
      background: var(--bg-secondary);
      border: 1px solid var(--border-color);
      border-radius: 8px;
      padding: 16px;
      margin-top: 16px;
    }

    .philosophy-panel h3 {
      color: var(--accent-brahman);
      margin-top: 0;
      font-size: 1rem;
    }

    .kosha-indicator {
      display: inline-flex;
      align-items: center;
      gap: 4px;
      padding: 2px 8px;
      border-radius: 4px;
      font-size: 0.75rem;
      background: var(--bg-tertiary);
    }

    .kosha-anna { border-left: 3px solid #ff6b6b; }
    .kosha-prana { border-left: 3px solid #ffa94d; }
    .kosha-mano { border-left: 3px solid #69db7c; }
    .kosha-vijnana { border-left: 3px solid #74c0fc; }
    .kosha-ananda { border-left: 3px solid #b197fc; }
  </style>
</head>
<body>
  <div class="header">
    <span class="om-symbol">🕉️</span>
    <h1>AST Viewer - वृक्ष दर्शन</h1>
  </div>

  <div class="toolbar">
    <button onclick="expandAll()">Expand All</button>
    <button onclick="collapseAll()">Collapse All</button>
    <button onclick="filterFunctions()" id="btn-functions">Functions</button>
    <button onclick="filterTypes()" id="btn-types">Types</button>
    <button onclick="showKoshas()">Show Kośas</button>
  </div>

  <div class="ast-container" id="ast-tree">
    <div class="empty-state">
      <div class="om">🕉️</div>
      <h3>Open a Jagannath file to view its AST</h3>
      <p>The Abstract Syntax Tree reveals the inner structure of your code,<br>
      like the subtle body (sūkṣma śarīra) underlying the gross form.</p>
    </div>
  </div>

  <div class="philosophy-panel">
    <h3>📊 Compilation Stage (Sāṃkhya Tattva)</h3>
    <p style="color: var(--text-secondary); font-size: 0.875rem;">
      Current: <strong>Parsing</strong> (Buddhi - Discriminative Intelligence)<br>
      The AST represents the mental form of your program before transformation.
    </p>
  </div>

  <script>
    const vscode = acquireVsCodeApi();

    function renderNode(node, depth = 0) {
      if (!node) return '';

      const children = node.children || [];
      const hasChildren = children.length > 0;
      const nodeClass = getNodeClass(node.type);

      return \`
        <div class="tree-node" data-depth="\${depth}">
          <div class="node-header" onclick="toggleNode(this)">
            <span class="toggle-icon">\${hasChildren ? '▼' : '•'}</span>
            <span class="node-type \${nodeClass}">\${node.type}</span>
            \${node.name ? \`<span class="node-name">\${node.name}</span>\` : ''}
            \${node.kosha ? \`<span class="kosha-indicator kosha-\${node.kosha}">\${node.kosha}</span>\` : ''}
            \${node.affixes ? \`<span class="node-badge">\${node.affixes}</span>\` : ''}
          </div>
          \${hasChildren ? \`<div class="tree-children">\${children.map(c => renderNode(c, depth + 1)).join('')}</div>\` : ''}
        </div>
      \`;
    }

    function getNodeClass(type) {
      if (/function|kāryakrama/i.test(type)) return 'function';
      if (/type|prakāra|struct|enum/i.test(type)) return 'type';
      if (/expr|literal|call/i.test(type)) return 'expression';
      if (/stmt|return|if|loop/i.test(type)) return 'statement';
      if (/error|naraka/i.test(type)) return 'error';
      return '';
    }

    function toggleNode(header) {
      const node = header.parentElement;
      node.classList.toggle('collapsed');
      const icon = header.querySelector('.toggle-icon');
      icon.textContent = node.classList.contains('collapsed') ? '▶' : '▼';
    }

    function expandAll() {
      document.querySelectorAll('.tree-node').forEach(n => n.classList.remove('collapsed'));
      document.querySelectorAll('.toggle-icon').forEach(i => i.textContent = '▼');
    }

    function collapseAll() {
      document.querySelectorAll('.tree-node').forEach(n => {
        if (n.querySelector('.tree-children')) {
          n.classList.add('collapsed');
        }
      });
      document.querySelectorAll('.toggle-icon').forEach(i => {
        if (i.textContent !== '•') i.textContent = '▶';
      });
    }

    window.addEventListener('message', event => {
      const message = event.data;
      if (message.type === 'updateAST') {
        const container = document.getElementById('ast-tree');
        if (message.ast) {
          container.innerHTML = renderNode(message.ast);
        }
      }
    });
  </script>
</body>
</html>`;
  }

  public dispose() {
    ASTViewerPanel.currentPanel = undefined;
    this._panel.dispose();
    while (this._disposables.length) {
      const x = this._disposables.pop();
      if (x) x.dispose();
    }
  }
}

// =============================================================================
// COMPILATION PIPELINE VIEWER
// =============================================================================

class PipelineViewerPanel {
  public static currentPanel: PipelineViewerPanel | undefined;
  private readonly _panel: vscode.WebviewPanel;
  private _disposables: vscode.Disposable[] = [];

  private constructor(panel: vscode.WebviewPanel, extensionUri: vscode.Uri) {
    this._panel = panel;
    this._panel.onDidDispose(() => this.dispose(), null, this._disposables);
    this._panel.webview.html = this._getHtmlContent();
  }

  public static createOrShow(extensionUri: vscode.Uri) {
    const column = vscode.ViewColumn.Two;

    if (PipelineViewerPanel.currentPanel) {
      PipelineViewerPanel.currentPanel._panel.reveal(column);
      return;
    }

    const panel = vscode.window.createWebviewPanel(
      "jagannathPipeline",
      "🔄 Compilation Pipeline - Sāṃkhya",
      column,
      { enableScripts: true, retainContextWhenHidden: true }
    );

    PipelineViewerPanel.currentPanel = new PipelineViewerPanel(
      panel,
      extensionUri
    );
  }

  public updateStage(stage: string, data: any) {
    this._panel.webview.postMessage({ type: "updateStage", stage, data });
  }

  private _getHtmlContent(): string {
    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Compilation Pipeline</title>
  <style>
    :root {
      --bg-primary: #0d1117;
      --bg-secondary: #161b22;
      --bg-tertiary: #21262d;
      --text-primary: #c9d1d9;
      --text-secondary: #8b949e;
      --accent-1: #ff6b6b;   /* Purusha - consciousness */
      --accent-2: #ffd93d;   /* Prakriti - matter */
      --accent-3: #6bcb77;   /* Mahat - cosmic mind */
      --accent-4: #4d96ff;   /* Ahamkara - ego */
      --accent-5: #a66cff;   /* Manas - mind */
      --accent-6: #ff922b;   /* Indriyas - senses */
      --accent-7: #20c997;   /* Tanmatras - elements */
      --border-color: #30363d;
    }

    body {
      font-family: 'Segoe UI', system-ui, sans-serif;
      background: var(--bg-primary);
      color: var(--text-primary);
      margin: 0;
      padding: 20px;
    }

    .header {
      text-align: center;
      margin-bottom: 30px;
    }

    .header h1 {
      font-size: 1.5rem;
      background: linear-gradient(135deg, var(--accent-1), var(--accent-5));
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }

    .header p {
      color: var(--text-secondary);
      font-size: 0.9rem;
    }

    .pipeline {
      display: flex;
      flex-direction: column;
      gap: 12px;
      max-width: 800px;
      margin: 0 auto;
    }

    .stage {
      display: flex;
      align-items: stretch;
      gap: 16px;
      background: var(--bg-secondary);
      border: 1px solid var(--border-color);
      border-radius: 8px;
      padding: 16px;
      transition: all 0.3s;
      cursor: pointer;
    }

    .stage:hover {
      border-color: var(--accent-4);
      transform: translateX(4px);
    }

    .stage.active {
      border-color: var(--accent-3);
      box-shadow: 0 0 20px rgba(107, 203, 119, 0.2);
    }

    .stage.completed {
      border-left: 4px solid var(--accent-3);
    }

    .stage.error {
      border-left: 4px solid var(--accent-1);
    }

    .stage-number {
      width: 48px;
      height: 48px;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      font-weight: bold;
      font-size: 1.2rem;
      flex-shrink: 0;
    }

    .stage:nth-child(1) .stage-number { background: var(--accent-1); color: white; }
    .stage:nth-child(2) .stage-number { background: var(--accent-2); color: black; }
    .stage:nth-child(3) .stage-number { background: var(--accent-3); color: white; }
    .stage:nth-child(4) .stage-number { background: var(--accent-4); color: white; }
    .stage:nth-child(5) .stage-number { background: var(--accent-5); color: white; }
    .stage:nth-child(6) .stage-number { background: var(--accent-6); color: white; }
    .stage:nth-child(7) .stage-number { background: var(--accent-7); color: white; }

    .stage-content {
      flex: 1;
    }

    .stage-title {
      font-weight: 600;
      font-size: 1.1rem;
      margin-bottom: 4px;
    }

    .stage-sanskrit {
      color: var(--text-secondary);
      font-style: italic;
      font-size: 0.9rem;
      margin-bottom: 8px;
    }

    .stage-description {
      color: var(--text-secondary);
      font-size: 0.85rem;
    }

    .stage-stats {
      display: flex;
      gap: 12px;
      margin-top: 8px;
    }

    .stat {
      background: var(--bg-tertiary);
      padding: 4px 8px;
      border-radius: 4px;
      font-size: 0.75rem;
    }

    .stat-value {
      font-weight: 600;
      color: var(--accent-4);
    }

    .flow-arrow {
      text-align: center;
      color: var(--text-secondary);
      font-size: 1.5rem;
    }

    .legend {
      margin-top: 30px;
      padding: 16px;
      background: var(--bg-secondary);
      border-radius: 8px;
    }

    .legend h3 {
      color: var(--accent-5);
      margin-top: 0;
    }

    .legend-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      gap: 12px;
    }

    .legend-item {
      display: flex;
      align-items: center;
      gap: 8px;
      font-size: 0.85rem;
    }

    .legend-color {
      width: 12px;
      height: 12px;
      border-radius: 50%;
    }
  </style>
</head>
<body>
  <div class="header">
    <h1>🔄 Compilation Pipeline - सांख्य तत्त्व</h1>
    <p>25 Tattvas of Sāṃkhya philosophy mapped to compilation stages</p>
  </div>

  <div class="pipeline">
    <div class="stage" id="stage-lexer">
      <div class="stage-number">1</div>
      <div class="stage-content">
        <div class="stage-title">Lexer - वर्ण विश्लेषण</div>
        <div class="stage-sanskrit">Prakriti → Mahat (Primordial matter → Cosmic mind)</div>
        <div class="stage-description">Transforms raw text into tokens using Sanskrit sandhi rules</div>
        <div class="stage-stats">
          <span class="stat">Tokens: <span class="stat-value" id="lexer-tokens">-</span></span>
          <span class="stat">Time: <span class="stat-value" id="lexer-time">-</span></span>
        </div>
      </div>
    </div>

    <div class="flow-arrow">↓</div>

    <div class="stage" id="stage-parser">
      <div class="stage-number">2</div>
      <div class="stage-content">
        <div class="stage-title">Parser - व्याकरण विश्लेषण</div>
        <div class="stage-sanskrit">Mahat → Ahaṃkāra (Cosmic mind → Individual ego)</div>
        <div class="stage-description">Builds AST using Pāṇinian grammar principles</div>
        <div class="stage-stats">
          <span class="stat">Nodes: <span class="stat-value" id="parser-nodes">-</span></span>
          <span class="stat">Depth: <span class="stat-value" id="parser-depth">-</span></span>
        </div>
      </div>
    </div>

    <div class="flow-arrow">↓</div>

    <div class="stage" id="stage-semantics">
      <div class="stage-number">3</div>
      <div class="stage-content">
        <div class="stage-title">Semantic Analysis - अर्थ विश्लेषण</div>
        <div class="stage-sanskrit">Ahaṃkāra → Manas (Ego → Mind)</div>
        <div class="stage-description">Type checking with Nyāya logic (4 pramāṇas)</div>
        <div class="stage-stats">
          <span class="stat">Types: <span class="stat-value" id="sem-types">-</span></span>
          <span class="stat">Errors: <span class="stat-value" id="sem-errors">-</span></span>
        </div>
      </div>
    </div>

    <div class="flow-arrow">↓</div>

    <div class="stage" id="stage-mir">
      <div class="stage-number">4</div>
      <div class="stage-content">
        <div class="stage-title">MIR Generation - मध्य प्रतिनिधित्व</div>
        <div class="stage-sanskrit">Manas → Indriyas (Mind → Senses)</div>
        <div class="stage-description">Mid-level IR with control flow graphs</div>
        <div class="stage-stats">
          <span class="stat">Blocks: <span class="stat-value" id="mir-blocks">-</span></span>
          <span class="stat">Instructions: <span class="stat-value" id="mir-instrs">-</span></span>
        </div>
      </div>
    </div>

    <div class="flow-arrow">↓</div>

    <div class="stage" id="stage-optimize">
      <div class="stage-number">5</div>
      <div class="stage-content">
        <div class="stage-title">Optimization - अस्त्र प्रयोग</div>
        <div class="stage-sanskrit">Deploy Divine Weapons (Astras)</div>
        <div class="stage-description">Brahmastra (DCE), Agneyastra (CPU), Varunastra (Memory)</div>
        <div class="stage-stats">
          <span class="stat">Astras: <span class="stat-value" id="opt-astras">-</span></span>
          <span class="stat">Reduction: <span class="stat-value" id="opt-reduction">-</span></span>
        </div>
      </div>
    </div>

    <div class="flow-arrow">↓</div>

    <div class="stage" id="stage-codegen">
      <div class="stage-number">6</div>
      <div class="stage-content">
        <div class="stage-title">Code Generation - संकलन</div>
        <div class="stage-sanskrit">Tanmātras → Mahābhūtas (Subtle → Gross elements)</div>
        <div class="stage-description">Direct assembly generation for x86_64/aarch64/riscv64</div>
        <div class="stage-stats">
          <span class="stat">Target: <span class="stat-value" id="cg-target">-</span></span>
          <span class="stat">Size: <span class="stat-value" id="cg-size">-</span></span>
        </div>
      </div>
    </div>

    <div class="flow-arrow">↓</div>

    <div class="stage" id="stage-output">
      <div class="stage-number">7</div>
      <div class="stage-content">
        <div class="stage-title">Binary Output - मोक्ष</div>
        <div class="stage-sanskrit">Liberation - Executable released into the world</div>
        <div class="stage-description">Final binary with 9 Durga security layers verified</div>
        <div class="stage-stats">
          <span class="stat">Binary: <span class="stat-value" id="out-binary">-</span></span>
          <span class="stat">Security: <span class="stat-value" id="out-security">-</span></span>
        </div>
      </div>
    </div>
  </div>

  <div class="legend">
    <h3>📚 Sāṃkhya Philosophy Mapping</h3>
    <div class="legend-grid">
      <div class="legend-item">
        <div class="legend-color" style="background: var(--accent-1);"></div>
        <span>Puruṣa (पुरुष) - Pure Consciousness</span>
      </div>
      <div class="legend-item">
        <div class="legend-color" style="background: var(--accent-2);"></div>
        <span>Prakṛti (प्रकृति) - Primordial Matter</span>
      </div>
      <div class="legend-item">
        <div class="legend-color" style="background: var(--accent-3);"></div>
        <span>Mahat (महत्) - Cosmic Intelligence</span>
      </div>
      <div class="legend-item">
        <div class="legend-color" style="background: var(--accent-4);"></div>
        <span>Ahaṃkāra (अहंकार) - Ego Principle</span>
      </div>
      <div class="legend-item">
        <div class="legend-color" style="background: var(--accent-5);"></div>
        <span>Manas (मनस्) - Mind</span>
      </div>
      <div class="legend-item">
        <div class="legend-color" style="background: var(--accent-6);"></div>
        <span>Indriyas (इन्द्रिय) - Sense Faculties</span>
      </div>
      <div class="legend-item">
        <div class="legend-color" style="background: var(--accent-7);"></div>
        <span>Bhūtas (भूत) - Gross Elements</span>
      </div>
    </div>
  </div>

  <script>
    const vscode = acquireVsCodeApi();

    function updateStage(stageId, status) {
      const stage = document.getElementById('stage-' + stageId);
      if (stage) {
        stage.classList.remove('active', 'completed', 'error');
        stage.classList.add(status);
      }
    }

    window.addEventListener('message', event => {
      const message = event.data;
      if (message.type === 'updateStage') {
        updateStage(message.stage, 'active');
        // Update stats if provided
        if (message.data) {
          Object.entries(message.data).forEach(([key, value]) => {
            const el = document.getElementById(key);
            if (el) el.textContent = value;
          });
        }
      }
    });

    // Click handlers
    document.querySelectorAll('.stage').forEach(stage => {
      stage.addEventListener('click', () => {
        vscode.postMessage({ type: 'stageClicked', stage: stage.id });
      });
    });
  </script>
</body>
</html>`;
  }

  public dispose() {
    PipelineViewerPanel.currentPanel = undefined;
    this._panel.dispose();
    while (this._disposables.length) {
      const x = this._disposables.pop();
      if (x) x.dispose();
    }
  }
}

// =============================================================================
// MAIN ACTIVATION
// =============================================================================

export function activate(context: vscode.ExtensionContext) {
  // Get server path from configuration or use default
  const config = vscode.workspace.getConfiguration("jagannath");
  let serverPath = config.get<string>("serverPath");

  if (!serverPath) {
    serverPath = context.asAbsolutePath(path.join("server", "jagannath-lsp"));
  }

  // Server options
  const serverOptions: ServerOptions = {
    run: { command: serverPath, transport: TransportKind.stdio },
    debug: { command: serverPath, transport: TransportKind.stdio },
  };

  // Client options
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "jagannath" }],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher(
        "**/*.{jag,jagannath}"
      ),
    },
  };

  // Create and start LSP client
  client = new LanguageClient(
    "jagannathLanguageServer",
    "Jagannath Language Server",
    serverOptions,
    clientOptions
  );

  // Register semantic tokens provider
  context.subscriptions.push(
    vscode.languages.registerDocumentSemanticTokensProvider(
      { language: "jagannath" },
      new JagannathSemanticTokensProvider(),
      legend
    )
  );

  // Register code lens provider
  context.subscriptions.push(
    vscode.languages.registerCodeLensProvider(
      { language: "jagannath" },
      new JagannathCodeLensProvider()
    )
  );

  // Register hover provider
  context.subscriptions.push(
    vscode.languages.registerHoverProvider(
      { language: "jagannath" },
      new JagannathHoverProvider()
    )
  );

  // Register commands
  context.subscriptions.push(
    vscode.commands.registerCommand("jagannath.build", buildCommand),
    vscode.commands.registerCommand("jagannath.run", runCommand),
    vscode.commands.registerCommand("jagannath.format", formatCommand),
    vscode.commands.registerCommand(
      "jagannath.invokeAstra",
      invokeAstraCommand
    ),
    vscode.commands.registerCommand("jagannath.yamaJudge", yamaJudgeCommand),
    vscode.commands.registerCommand(
      "jagannath.selectMarga",
      selectMargaCommand
    ),
    vscode.commands.registerCommand("jagannath.showAstViewer", () => {
      ASTViewerPanel.createOrShow(context.extensionUri);
    }),
    vscode.commands.registerCommand("jagannath.showPipeline", () => {
      PipelineViewerPanel.createOrShow(context.extensionUri);
    }),
    vscode.commands.registerCommand(
      "jagannath.analyzeFunction",
      (name, uri, line) => {
        vscode.window.showInformationMessage(`🕉️ Analyzing function: ${name}`);
        ASTViewerPanel.createOrShow(context.extensionUri);
      }
    ),
    vscode.commands.registerCommand(
      "jagannath.viewMemoryLayout",
      (name, uri, line) => {
        vscode.window.showInformationMessage(`📊 Memory layout for: ${name}`);
      }
    ),
    vscode.commands.registerCommand(
      "jagannath.viewAstraOptimization",
      (uri, line) => {
        vscode.window.showInformationMessage(
          `⚔️ Astra optimization analysis at line ${line + 1}`
        );
      }
    ),
    vscode.commands.registerCommand(
      "jagannath.viewNarakaClassification",
      (uri, line) => {
        vscode.window.showInformationMessage(
          `🔥 Naraka error classification at line ${line + 1}`
        );
      }
    )
  );

  // Register diagnostics provider (simple Naraka mapper)
  const diagnosticCollection =
    vscode.languages.createDiagnosticCollection("jagannath-naraka");
  context.subscriptions.push(diagnosticCollection);

  context.subscriptions.push(
    vscode.workspace.onDidSaveTextDocument((doc) => {
      if (doc.languageId !== "jagannath") return;
      const diagnostics: vscode.Diagnostic[] = [];
      const text = doc.getText();

      // Simple patterns mapping to Naraka types
      const patterns: {
        regex: RegExp;
        naraka: string;
        severity: vscode.DiagnosticSeverity;
        message: string;
      }[] = [
        {
          regex: /use-after-free/i,
          naraka: "Tamisram",
          severity: vscode.DiagnosticSeverity.Error,
          message: "Use-after-free detected (Tamisram)",
        },
        {
          regex: /null pointer/i,
          naraka: "Andhakupa",
          severity: vscode.DiagnosticSeverity.Error,
          message: "Null pointer dereference (Andhakupa)",
        },
        {
          regex: /tainted data/i,
          naraka: "Vaitarani",
          severity: vscode.DiagnosticSeverity.Warning,
          message: "Tainted data crossing boundary (Vaitarani)",
        },
        {
          regex: /memory leak/i,
          naraka: "Suchimukha",
          severity: vscode.DiagnosticSeverity.Warning,
          message: "Possible memory leak (Suchimukha)",
        },
        {
          regex: /buffer overflow/i,
          naraka: "Asipatravana",
          severity: vscode.DiagnosticSeverity.Error,
          message: "Buffer overflow risk (Asipatravana)",
        },
      ];

      const lines = text.split(/\r?\n/);
      for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        for (const p of patterns) {
          const m = p.regex.exec(line);
          if (m) {
            const range = new vscode.Range(
              i,
              m.index,
              i,
              m.index + m[0].length
            );
            const diag = new vscode.Diagnostic(
              range,
              `${p.message} — Naraka: ${p.naraka}. Penance: apply śuddhi-kri()`,
              p.severity
            );
            diag.source = "Yama";
            diag.code = p.naraka;
            diagnostics.push(diag);
          }
        }
      }

      diagnosticCollection.set(doc.uri, diagnostics);
    })
  );

  // Register Naraka code actions and penance command
  registerNarakaProviders(context);

  // Start the LSP client
  client.start();

  // Listen for document changes to update viewers
  context.subscriptions.push(
    vscode.workspace.onDidChangeTextDocument((event) => {
      if (event.document.languageId === "jagannath") {
        // Could trigger AST/pipeline updates here
      }
    })
  );

  console.log("🕉️ Jagannath extension activated with enhanced features");
  vscode.window.showInformationMessage("🕉️ Jagannath Language - Om Tat Sat");
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}

// =============================================================================
// BUILD/RUN COMMANDS
// =============================================================================

async function buildCommand() {
  const terminal = vscode.window.createTerminal("Jagannath Build");
  const editor = vscode.window.activeTextEditor;

  if (editor && editor.document.languageId === "jagannath") {
    const filePath = editor.document.fileName;
    const config = vscode.workspace.getConfiguration("jagannath");
    const guna = config.get<string>("guna", "rajas");

    terminal.sendText(`jagannath build --${guna} "${filePath}"`);
    terminal.show();

    // Update pipeline viewer if open
    if (PipelineViewerPanel.currentPanel) {
      PipelineViewerPanel.currentPanel.updateStage("lexer", {
        "lexer-time": "starting...",
      });
    }
  } else {
    vscode.window.showErrorMessage("No Jagannath file is open");
  }
}

async function runCommand() {
  const terminal = vscode.window.createTerminal("Jagannath Run");
  const editor = vscode.window.activeTextEditor;

  if (editor && editor.document.languageId === "jagannath") {
    const filePath = editor.document.fileName;
    terminal.sendText(`jagannath run "${filePath}"`);
    terminal.show();
  } else {
    vscode.window.showErrorMessage("No Jagannath file is open");
  }
}

async function formatCommand() {
  const editor = vscode.window.activeTextEditor;

  if (editor && editor.document.languageId === "jagannath") {
    await vscode.commands.executeCommand("editor.action.formatDocument");
  } else {
    vscode.window.showErrorMessage("No Jagannath file is open");
  }
}

// =============================================================================
// NEW COMMAND HANDLERS
// =============================================================================

async function invokeAstraCommand() {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== "jagannath") {
    vscode.window.showErrorMessage("Open a Jagannath file to invoke an Astra");
    return;
  }

  const astra = await vscode.window.showQuickPick(
    [
      "Brahmastra",
      "Agneyastra",
      "Varunastra",
      "Pashupatastra",
      "SudarshanaChakra",
    ],
    { placeHolder: "Select Astra to deploy (optimization)" }
  );

  if (!astra) return;

  const mantra = await vscode.window.showInputBox({
    prompt: `Enter mantra for ${astra} (or leave empty for default)`,
  });
  vscode.window.showInformationMessage(
    `Invoking ${astra} ${
      mantra ? `with mantra: ${mantra}` : "with default mantra"
    }`
  );
}

async function yamaJudgeCommand() {
  const editor = vscode.window.activeTextEditor;
  if (!editor || editor.document.languageId !== "jagannath") {
    vscode.window.showErrorMessage("Open a Jagannath file to run Yama Judge");
    return;
  }

  // Run a lightweight static analysis using existing diagnostics
  await editor.document.save();
  vscode.window.showInformationMessage(
    "Yama Judge: Static analysis completed — check Problems panel for Naraka diagnostics"
  );
}

async function selectMargaCommand() {
  const options = [
    { label: "karma", description: "Action: optimize imperative code" },
    { label: "jnana", description: "Knowledge: optimize pure code" },
    { label: "bhakti", description: "Devotion: domain-specific" },
    { label: "raja", description: "Royal: balanced hybrid" },
  ];
  const pick = await vscode.window.showQuickPick(options, {
    placeHolder: "Select Mārga (optimization path)",
  });
  if (!pick) return;
  const config = vscode.workspace.getConfiguration("jagannath");
  await config.update(
    "marga",
    pick.label,
    vscode.ConfigurationTarget.Workspace
  );
  vscode.window.showInformationMessage(`Mārga selected: ${pick.label}`);
}

// =============================================================================
// CODE ACTION PROVIDER - Quick fixes for Naraka diagnostics
// =============================================================================

class NarakaCodeActionProvider implements vscode.CodeActionProvider {
  public provideCodeActions(
    document: vscode.TextDocument,
    range: vscode.Range
  ): vscode.CodeAction[] | undefined {
    const diagnostics = vscode.languages
      .getDiagnostics(document.uri)
      .filter((d) => d.source === "Yama" && range.intersection(d.range));
    if (!diagnostics || diagnostics.length === 0) return;

    const actions: vscode.CodeAction[] = [];
    for (const diag of diagnostics) {
      const naraka = diag.code as string;
      const title = `Apply penance for ${naraka}`;
      const action = new vscode.CodeAction(
        title,
        vscode.CodeActionKind.QuickFix
      );
      action.command = {
        command: "jagannath.applyPenance",
        title,
        arguments: [document.uri, diag.range, naraka],
      };
      action.diagnostics = [diag];
      actions.push(action);
    }

    return actions;
  }
}

function registerNarakaProviders(context: vscode.ExtensionContext) {
  context.subscriptions.push(
    vscode.languages.registerCodeActionsProvider(
      { language: "jagannath" },
      new NarakaCodeActionProvider(),
      { providedCodeActionKinds: [vscode.CodeActionKind.QuickFix] }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "jagannath.applyPenance",
      async (uri: vscode.Uri, range: vscode.Range, naraka: string) => {
        const doc = await vscode.workspace.openTextDocument(uri);
        const editor = await vscode.window.showTextDocument(doc);
        // Simple penance insertion: add a comment with recommended fix
        await editor.edit((edit) => {
          edit.insert(
            new vscode.Position(range.end.line + 1, 0),
            `// PRĀYAŚCITTA: apply śuddhi-kri() to resolve ${naraka}\n`
          );
        });
        vscode.window.showInformationMessage(`Penance applied for ${naraka}`);
      }
    )
  );
}
