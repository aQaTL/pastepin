"use strict";

const api_url = "/a";

const parseFilename = name =>
	name === "" || name === null || name === undefined ? "unnamed" : name;

window.onload = () => {
	let app = new Vue({
		el: '#app',
		data: {
			page: 0,
			total_pages: 1,
			pastes: [],
		},
		methods: {
			infiniteHandler($state) {
				if (this.page < this.total_pages) {
					let req = new XMLHttpRequest();
					req.open("GET", `${api_url}?page=${this.page + 1}`, true);
					req.onload = () => {
						if (req.status !== 200) {
							$state.complete();
							return;
						}
						let paginatedPastes = JSON.parse(req.responseText);
						this.page = paginatedPastes.page;
						this.total_pages = paginatedPastes.total_pages;
						paginatedPastes.results
							.forEach(paste => {
								paste.filename = parseFilename(paste.filename);
								paste.content = paste.content.split("\n", 4).join("\n");
								this.pastes.push(paste);
							});
						$state.loaded();
					};
					req.send();
				} else {
					$state.complete();
				}
			},
		}
	});
};