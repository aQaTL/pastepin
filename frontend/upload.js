"use strict";

const parseFilename = filename => filename === "" || filename === null || filename === undefined ? "unnamed" : filename;

Vue.component("uploaded-paste", {
	props: ["paste"],

	data: function () {
		return {
			lang: window.navigator.language,
		};
	},

	filters: {
		parseFilename: parseFilename,
	},

	mounted: function () {
		this.$el.scrollIntoView();
	},

	template: `<div class="card fluid">
			<h4><a :href="paste.id" target="_blank">{{ paste.filename | parseFilename }}</a>
			<small>Uploaded: {{ paste.creation_date.toLocaleString(lang) }}</small>
			</h4>
			<p>Id: {{ paste.id }}</p>
		</div>
	`
});


Vue.component("uploaded-image", {
	props: ["image"],

	data: function () {
		return {
			lang: window.navigator.language,
		};
	},

	filters: {
		parseFilename: parseFilename,
	},

	mounted: function () {
		this.$el.scrollIntoView();
	},

	template: `<div class="card fluid">
			<h4 v-if="image.success"><a :href="'/i/'+image.id" target="_blank">{{ image.filename | parseFilename }}</a>
			</h4>
			<h4 v-else>{{ image.filename }} failed</h4>
			<p v-if="image.success">Id: {{ image.id }}</p>
			<p v-else>Reason: {{ image.err }}</p>
		</div>
	`
});

Vue.component("uploaded", {
	props: ["paste"],

	template: `<uploaded-paste v-if="paste.type === 'txt'" v-bind:paste="paste.data"></uploaded-paste>
<uploaded-image v-else-if="paste.type === 'img'" v-bind:image="paste.data"></uploaded-image>`,
});

window.onload = () => {
	let app = new Vue({
		el: '#app',

		data: {
			uploaded: [],
			internalId: 0,
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
					self.uploaded.push({
						type: "txt",
						internalId: self.internalId++,
						data: {
							id: paste.id,
							filename: form.filename.value,
							creation_date: new Date(paste.creation_date + "Z"),
						},
					});
				};
				req.setRequestHeader("Content-type", "application/json");
				req.send(JSON.stringify({
					filename: form.filename.value,
					content: form.content.value,
				}));
			},
			sendImg: function () {
				let form = document.imageForm;
				let req = new XMLHttpRequest();
				let self = this;

				req.open("POST", "/ui", true);
				req.onload = function () {
					if (req.status !== 201) {
						showSnackbar("Failed to upload");
						console.log(req.responseText);
						return;
					}
					let ids = JSON.parse(req.responseText);
					ids.forEach((resp, idx) => {
						if (resp.hasOwnProperty("Ok")) {
							self.uploaded.push({
								type: "img",
								internalId: self.internalId++,
								data: {
									id: resp.Ok,
									filename: form.image.files[idx].name,
									success: true,
								},
							});
						} else {
							self.uploaded.push({
								type: "img",
								internalId: self.internalId++,
								data: {
									filename: form.image.files[idx].name,
									err: resp.Err.err,
									success: false,
								},
							});
							console.log(`${form.image.files[idx].name} failed, reason: ${resp.Err.err}`);
						}
					});
				};
				req.send(new FormData(document.imageForm));
			},
		}
	});
};