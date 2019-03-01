"use strict";

const api_url = "/a";

window.onload = () => {
	let app = new Vue({
		el: '#app',

		data: {
			page: 1,
			pastes: null,
		},

		created: function () {
			this.fetchData();
		},

		filters: {
			parseFilename: filename =>
				filename === "" || filename === null || filename === undefined ? "unnamed" : filename,
		},

		methods: {
			fetchData: function () {
				let self = this;
				let req = new XMLHttpRequest();
				req.open("GET", api_url);
				req.onload = () => self.pastes = JSON.parse(req.responseText);
				req.send();
			}
		}
	});
};