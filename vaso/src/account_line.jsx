var __extends = (this && this.__extends) || (function () {
    var extendStatics = Object.setPrototypeOf ||
        ({ __proto__: [] } instanceof Array && function (d, b) { d.__proto__ = b; }) ||
        function (d, b) { for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p]; };
    return function (d, b) {
        extendStatics(d, b);
        function __() { this.constructor = d; }
        d.prototype = b === null ? Object.create(b) : (__.prototype = b.prototype, new __());
    };
})();
var __decorate = (this && this.__decorate) || function (decorators, target, key, desc) {
    var c = arguments.length, r = c < 3 ? target : desc === null ? desc = Object.getOwnPropertyDescriptor(target, key) : desc, d;
    if (typeof Reflect === "object" && typeof Reflect.decorate === "function") r = Reflect.decorate(decorators, target, key, desc);
    else for (var i = decorators.length - 1; i >= 0; i--) if (d = decorators[i]) r = (c < 3 ? d(r) : c > 3 ? d(target, key, r) : d(target, key)) || r;
    return c > 3 && r && Object.defineProperty(target, key, r), r;
};
import Vue from 'vue';
import Component from 'vue-class-component';
var template = "\n<div class=\"account-list-item\" style=\"\n    max-width: 500px; \">\n    <div class=\"currency-icon\" style=\"margin: 15px;\">\n        <i class=\"fas fa-money-bill-alt\" aria-hidden=\"true\" style=\"font-size: 40px;\"></i>\n    </div>\n<div class=\"account-description\" style=\"padding: 15px;\">\n    <div style=\"font-size: 12pt; \">Cash Wallet (NZD)</div>\n    <a href=\"javascript: void(0)\">Credit account</a>\n    <a href=\"javascript: void(0)\">Cash out</a>\n    <a href=\"javascript: void(0)\">Transactions</a>\n  </div> \n<div class=\"account-summary\" style=\"padding: 15px; margin-left: auto; \">\n    <div style=\"font-size: 12pt; \">$10.30</div>\n    <div>$5.30 available</div>\n  </div>\n</div>";
var AccountLine = /** @class */ (function (_super) {
    __extends(AccountLine, _super);
    function AccountLine() {
        return _super !== null && _super.apply(this, arguments) || this;
    }
    AccountLine.prototype.data = function () {
        return {
            account: null
        };
    };
    AccountLine = __decorate([
        Component({
            template: template,
            data: {},
            name: 'account-line',
            props: []
        })
    ], AccountLine);
    return AccountLine;
}(Vue));
export { AccountLine };
//# sourceMappingURL=account_line.js.map