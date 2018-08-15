import * as React from 'react';

class Props {
    value: string;
}

export function ClipboardCopy(props: Props) {
    return <span onClick={(e: any) => copyToClipboard(e, props.value)} className="clipboard-copy">
        <input type="text" defaultValue={props.value} style={{display: 'none'}}/>
        <i className="fas fa-copy"/>
    </span>;
}

function copyToClipboard(e: any, value: string) {
    console.log('copy this bitch!', e, value);
}
