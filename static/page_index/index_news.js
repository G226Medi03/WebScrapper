import HttpClient from "/static/HttpClient.js";

function get_NewsCard(title, link, imgSrc) {
    return `<news-card link="${link}"
                imgSrc="${imgSrc}"
                title="${title}">
            </news-card>`
}

function get_NewsCards(arr) {
    let html = ``;

    arr.forEach((elem) => {
        html += get_NewsCard(elem["title"], elem["link"], elem["imgSrc"]);
    });

    return html;
}

function get_NewsSection(obj) {

    let html = `<news-section section=${obj["section"]}>
            ${get_NewsCards(obj["data"])}
        </news-section>`;
    return html;
}

function get_NewsSections(jData) {
    let html = ``;
    jData.forEach((obj) => {
        html += get_NewsSection(obj);
    });
    return html;
}


export default async function updateNewsCards() {
    let client = new HttpClient();

    client.getUrl("/news", (data) => {
        let sectionHtml = get_NewsSections(JSON.parse(data));
       
        document.querySelector(".newsContainer").innerHTML = sectionHtml;
    })

}

// updateNewsCards();
