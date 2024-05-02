// noinspection JSUnresolvedReference

let FrugurtLang = null;
let FrugurtParser = null;

window.TreeSitter.init().then(async () => {
    if (FrugurtParser == null) {
        FrugurtParser = new window.TreeSitter();
        FrugurtLang = await window.TreeSitter.Language.load("../highlighter/tree-sitter-frugurt.wasm");
        FrugurtParser.setLanguage(FrugurtLang);
        window.highlighterInfo.query = FrugurtLang.query(window.highlighterInfo.queryString);
    }

    let targets = document.getElementsByClassName("language-frugurt");

    for (let x = 0; x < targets.length; x++) {
        let text = targets[x].innerText;

        let tree = FrugurtParser.parse(text);

        let ranges = window.highlighterInfo.query.captures(tree.rootNode).map(x => (
            {
                from:  x.node.startIndex,
                to:    x.node.endIndex,
                color: window.highlighterInfo.colors[x.name],
            }
        ));

        targets[x].innerHTML = colorString(text, ranges);
    }
});


/** @param {string} str
 @param {{from: number, to: number, color: string}[]} ranges */
function colorString(str, ranges) {
    let result = "";

    let last = 0;

    for (let r in ranges) {
        result += str.slice(last, ranges[r].from);

        result += `<span style="color: ${ranges[r].color}">`;
        result += str.slice(ranges[r].from, ranges[r].to);
        result += "</span>";

        last = ranges[r].to;
    }

    result += str.slice(ranges[ranges.length - 1].to);

    return result;
}
