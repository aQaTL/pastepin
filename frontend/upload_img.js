"use strict";

Vue.component("uploaded-image", {
	props: ["image"],

	data: function () {
		return {
			lang: window.navigator.language,
		};
	},

	filters: {
		parseFilename: filename =>
			filename === "" || filename === null || filename === undefined ? "unnamed" : filename,
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

window.onload = () => {
	let app = new Vue({
		el: '#app',

		data: {
			uploadedImages: [],
		},

		methods: {
			send: function () {
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
							self.uploadedImages.push({
								id: resp.Ok,
								filename: form.image.files[idx].name,
								success: true,
							});
						} else {
							self.uploadedImages.push({
								filename: form.image.files[idx].name,
								err: resp.Err.err,
								success: false,
							});
							console.log(`${form.image.files[idx].name} failed, reason: ${resp.Err.err}`);
						}
					});
				};
				// req.setRequestHeader("Content-type", "multipart/form-data");
				req.send(new FormData(document.imageForm));
			},
		}
	});
};