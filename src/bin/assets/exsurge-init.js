let ctxt = new exsurge.ChantContext();
ctxt.lyricTextFont = "'Crimson Text', serif";
ctxt.lyricTextSize *= 1.2;
ctxt.dropCapTextFont = ctxt.lyricTextFont;
ctxt.annotationTextFont = ctxt.lyricTextFont;

var chantContainers = document.getElementsByClassName("chant-container");
for (let i=0; i< chantContainers.length; i++) {
    let gabc = chantContainers[i].textContent;
    let mappings = exsurge.Gabc.createMappingsFromSource(ctxt, gabc);
    let score = new exsurge.ChantScore(ctxt, mappings, true);
    score.annotation = new exsurge.Annotation(ctxt, "%V%");

    score.performLayoutAsync(ctxt, function() {
        score.layoutChantLines(ctxt, chantContainers[i].clientWidth, function() {
            // render the score to svg code
            chantContainers[i].innerHTML = score.createSvg(ctxt);
        });
    });
}