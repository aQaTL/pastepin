"use strict";

function showSnackbar(msg) {
    let sb = document.getElementById("snackbar");
    sb.innerHTML = msg;
    sb.className = "show";
    setTimeout(() => sb.className = "", 3000);
}

const snackbarFn = (msg) => () => showSnackbar(msg);

function copyToClipboard(elementId) {
    if (navigator.clipboard.writeText === undefined)
        showSnackbar("Clipboard not available");
    else
        navigator.clipboard.writeText(document.getElementById(elementId).innerText)
            .then(snackbarFn("Content copied!"), snackbarFn("Clipboard not available"));
}

function pasteFromClipboard(elementId) {
    if (navigator.clipboard.readText === undefined)
        showSnackbar("Clipboard not available");
    else
        navigator.clipboard.readText().then(
            (paste) => document.getElementById(elementId).innerText = paste,
            snackbarFn("Clipboard not available"));
}