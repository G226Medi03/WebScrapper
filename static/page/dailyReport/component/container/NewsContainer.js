import "/static/component/NaverNews/News.js";

class NewsContainer extends HTMLElement {
    constructor() {
        super();
    }

    createNewsCard({ title, link, imgSrc }) {
        return `
            <news-card 
                link="${link}" 
                imgSrc="${imgSrc}" 
                title="${title}">
            </news-card>`;
    }

    createNewsCards(newsArray) {
        return newsArray.map(this.createNewsCard).join('');
    }

    createNewsSection({ section, data }) {
        return `
            <news-section section="${section}">
                ${this.createNewsCards(data)}
            </news-section>`;
    }

    createNewsSections(newsData) {
        return newsData.map(this.createNewsSection.bind(this)).join('');
    }

    getHtml() {
        return `
            <div class="mb_15">
                <span class="fs_30">News</span>
            </div>
            <div class="Row newsContainer mac O_F"></div>`;
    }

    getCss() {
        return `
            .newsContainer {
                gap: 30px;
                height: 952px;
            }`;
    }

    async updateNewsCards() {
        try {
            const response = await fetch("/api/news");
            const data = await response.json();

            const sectionHtml = this.createNewsSections(data);
            this.querySelector(".newsContainer").innerHTML = sectionHtml;
        } catch (error) {
            console.error("Failed to update news cards:", error);
        }
    }

    connectedCallback() {
        const template = document.createElement("template");
        template.innerHTML = `
            <style>${this.getCss()}</style>
            ${this.getHtml()}`;

        this.appendChild(template.content.cloneNode(true));
        this.updateNewsCards();
    }
}

window.customElements.define("news-container", NewsContainer);