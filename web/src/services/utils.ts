
// This function generates a temporary URL
// for a blob file send from the API
export function getTempURL(data: any, type: string): string {
    let blob = new Blob([data], {type: type});
    var urlCreator = window.URL || window.webkitURL;
    var imageUrl = urlCreator.createObjectURL(blob);
    return imageUrl;
}
