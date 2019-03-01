"use strict";

Vue.component("uploaded-paste", {
	props: ["paste"],

	data: function () {
		return {
			lang: window.navigator.language,
		};
	},

	filters: {
		parseFilename: filename =>
			filename === "" || filename === null || filename === undefined ? "unnamed" : filename,
	},

	template: `<div class="card fluid">
			<h4><a :href="paste.id" target="_blank">{{ paste.filename | parseFilename }}</a>
			<small>Uploaded: {{ paste.creation_date.toLocaleString(lang) }}</small>
			</h4>
			<p>Id: {{ paste.id }}</p>
		</div>
	`
});

window.onload = () => {
	let app = new Vue({
		el: '#app',

		data: {
			uploadedPastes: [],
		},

		methods: {
			send: function () {
				let form = document.pasteForm;
				let req = new XMLHttpRequest();
				let self = this;

				req.open("POST", "/u", true);
				req.onload = function () {
					if (req.status !== 201) {
						showSnackbar("Failed to upload");
						return;
					}
					let paste = JSON.parse(req.responseText);
					self.uploadedPastes.push({
						id: paste.id,
						filename: form.filename.value,
						creation_date: new Date(paste.creation_date + "Z"),
					});
				};
				req.setRequestHeader("Content-type", "application/json");
				req.send(JSON.stringify({
					filename: form.filename.value,
					content: form.content.value,
				}));
			},
		}
	});
};