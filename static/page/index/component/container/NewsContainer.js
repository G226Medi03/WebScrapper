import _2 from "/static/component/NaverNews/News.js";
class NewsContainer extends HTMLElement {
    constructor() {
        super();
    }

    get_NewsCard(title, link, imgSrc) {
        return `<news-card link="${link}"
                imgSrc="${imgSrc}"
                title="${title}">
            </news-card>`
    }

    get_NewsCards(arr) {
        let html = ``;

        arr.forEach((elem) => {
            html += this.get_NewsCard(elem["title"], elem["link"], elem["imgSrc"]);
        });

        return html;
    }

    get_NewsSection(obj) {

        let html = `<news-section section=${obj["section"]}>
            ${this.get_NewsCards(obj["data"])}
        </news-section>`;
        return html;
    }

    get_NewsSections(jData) {
        let html = ``;
        jData.forEach((obj) => {
            html += this.get_NewsSection(obj);
        });
        return html;
    }


    getHtml() {
        return `<span class="MTB_10 FS_30">
                    News
                </span>
                <div class="Row newsContainer MAC O_F">
                </div>`;
    }
    getCss() {
        const style = `
            .newsContainer {
                gap : 30px;
                height : 952px;
            }`;

        return style;
    }

    async updateNewsCards() {



        await fetch("/news").then(resp => { return resp.text(); }).then(data => {
            let sectionHtml = this.get_NewsSections(JSON.parse(data));

            this.querySelector(".newsContainer").innerHTML = sectionHtml;
        })

    }


    connectedCallback() {
        const template = document.createElement("template");
        template.innerHTML = `
        <style>
            ${this.getCss()}
        </style>
        ${this.getHtml()}`;

        this.appendChild(template.content.cloneNode(true));
        this.updateNewsCards();
    }



}

export default async function Define() {
    window.customElements.define("news-container", NewsContainer);
}



Define();