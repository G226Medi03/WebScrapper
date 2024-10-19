
class NaverFinanceLogo extends HTMLElement {
    constructor() {
        super();
    }
    getHtml() {

        let width = this.getAttribute("width") || "100px";
        let height = this.getAttribute("height") || "25px";
        let onclick = this.getAttribute("onclick") || "";
        let svgColor = this.getAttribute("svgColor") || "white";

        return `<div class="NaverFinanceLogo" style="height: ${height}; width: ${width}; onclick=${onclick}">
                    <svg width="22" height="22" viewBox="0 0 22 22" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path
                                d="M22 11C22 13.1756 21.3549 15.3023 20.1462 17.1113C18.9375 18.9202 17.2195 20.3301 15.2095 21.1627C13.1995 21.9952 10.9878 22.2131 8.85401 21.7886C6.72022 21.3642 4.76021 20.3166 3.22183 18.7782C1.68345 17.2398 0.635805 15.2798 0.211368 13.146C-0.213069 11.0122 0.00476298 8.80047 0.837327 6.79048C1.66989 4.78049 3.07979 3.06253 4.88873 1.85383C6.69768 0.645137 8.82441 0 11 0C13.9174 0 16.7153 1.15893 18.7782 3.22183C20.8411 5.28473 22 8.08262 22 11ZM12.8398 5.83V11.3602L8.98976 5.83H5.83V16.1838H9.1575V10.637L13.0075 16.17H16.1783V5.83H12.8398Z"
                                fill="white">
                        </path>
                    </svg>
                    <div style="width: 7px;"></div>
                    <svg width="34" height="22" viewBox="0 0 34 22" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path
                            d="M2.43006 8.88998L1.85406 6.99998C5.29206 6.31598 7.00206 5.34398 7.45206 4.31798H2.71806V2.35598H14.3641V4.31798H9.75606C9.70206 4.65998 9.61206 4.98398 9.46806 5.28998L15.2101 6.98198L14.4901 8.88998L8.02806 6.92798C6.84006 7.75598 5.00406 8.40398 2.43006 8.88998ZM0.612061 11.5V9.53798H16.4521V11.5H0.612061ZM6.53406 12.796H10.5301C13.2301 12.796 14.7961 14.128 14.7961 15.91C14.7961 17.71 13.2301 19.024 10.5301 19.024H6.53406C3.85206 19.024 2.28606 17.71 2.28606 15.91C2.28606 14.128 3.85206 12.796 6.53406 12.796ZM6.96606 17.044H10.1161C11.7901 17.044 12.4921 16.594 12.4921 15.892C12.4921 15.19 11.7901 14.776 10.1161 14.776H6.96606C5.29206 14.776 4.57206 15.19 4.57206 15.892C4.57206 16.594 5.29206 17.044 6.96606 17.044Z"
                            fill="${svgColor}">
                        </path>
                        <path
                                d="M25.1904 13.354V11.536H29.4564V2.03198H31.7604V15.334H29.4564V13.354H25.1904ZM21.3384 10.564C20.9244 10.582 20.5464 10.582 20.1684 10.582H17.4504L17.3604 8.61998H20.3844C21.5004 8.61998 22.9584 8.56598 24.4884 8.45798C24.7224 6.99998 24.8484 5.63198 24.8664 4.62398H18.4404V2.66198H27.0984V3.56198C27.0984 5.16398 26.9724 6.76598 26.7564 8.25998L28.2684 8.09798L28.5024 10.006C26.9724 10.222 25.2264 10.384 23.5704 10.474C23.4984 11.5 23.3904 12.562 23.2644 13.714L21.0684 13.588C21.1764 12.508 21.2664 11.536 21.3384 10.564ZM22.4724 14.254V16.756H32.3724V18.844H20.1684V14.254H22.4724Z"
                                fill="${svgColor}">
                        </path>
                    </svg>
           </div>`;
    }
    getCss() {
        let backgroundColor = this.getAttribute("backgroundColor") || "#03B351";
        const style = `
            .NaverFinanceLogo {
                display: flex;
                flex-direction: row;

                margin: 10px;
                background-color: ${backgroundColor};
                padding: 7px;

                cursor: pointer;

                border-radius: 5px;

                justify-content: center;
                text-align: center;

                align-items: center;

                white-space: nowrap;
            }`;

        return style;
    }

    connectedCallback() {
        const template = document.createElement("template");
        template.innerHTML = `
        <style>
            ${this.getCss()}
        </style>
        ${this.getHtml()}`;

        this.appendChild(template.content.cloneNode(true));
    }
}

export default async function Define() {
    window.customElements.define("naverfinance-logo", NaverFinanceLogo);
}



Define();