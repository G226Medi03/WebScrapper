export default class HttpClient {
  constructor() {
    this.worker = new XMLHttpRequest();
  }

  getUrl(sUrl, sMsgHandleFunction) {
    this.worker.open("GET", sUrl);
    this.worker.send();

    this.worker.onreadystatechange = (e) => {
      if (this.worker.readyState == XMLHttpRequest.DONE) {
        sMsgHandleFunction(this.worker.responseText);
      }
    };
  }
}
