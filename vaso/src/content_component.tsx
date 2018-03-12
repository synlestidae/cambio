import * as React from "react";
import {SignupPage} from './signup_page';

export function ContentComponent() {
    return <div id="main-content">
      <div className="page-container signup-container">
        <SignupPage></SignupPage>
      </div>
    </div>;
}
