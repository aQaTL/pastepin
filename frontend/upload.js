"use strict";

function send() {
    let form = document.pasteForm;
    let req = new XMLHttpRequest();
    req.open("POST", "/u", true);
    req.onload = pasteSent;
    req.setRequestHeader("Content-type", "application/json");
    req.send(JSON.stringify({
        filename: form.filename.value,
        content: form.content.value,
    }));
}

function pasteSent() {
    if (this.status !== 201) {
        alert("Uploading failed");
        return;
    }
    let id = JSON.parse(this.responseText).id;
    document.getElementById("result").innerHTML =
        `Paste id: <a href="/p/${id}">${id}</a>`;
}
