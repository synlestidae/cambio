export function setCookie(key: string, value: string, expiry?: Date) {
    let cookie = `${key}=${value};`;
    if (expiry) {
        cookie = `${cookie} expires=${expiry.toUTCString()}`;
    }
    document.cookie = cookie;
}

export function getCookie(key: string): string|null {
    let nameEQ = name + "=";
    let ca = document.cookie.split(';');
    for (let i=0;i < ca.length;i++) {
        let c = ca[i];
        let [cKey, val] = c.split('=');
        if (cKey === key) {
            return val;
        }
    }
    return null;
}
