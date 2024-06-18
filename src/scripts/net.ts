export function request(path: string): Promise<number> {
  return new Promise((resolve, reject) => {
    var xhr = new XMLHttpRequest();
    xhr.open("GET", "http://localhost:26781/" + path, true);
    xhr.responseType = "text";

    xhr.onload = function () {
      if (xhr.status == 200) {
        resolve(Number.parseInt(xhr.responseText));
      } else {
        reject("error");
      }
    };

    xhr.send();
  });
}