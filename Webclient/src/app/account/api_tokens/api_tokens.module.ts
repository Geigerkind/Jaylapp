import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { ConfirmButtonModule } from "src/app/template/confirm_button/confirm_button.module";
import { GeneralInputModule } from "src/app/template/general_input/general_input.module";
import { APITokensComponent } from "./api_tokens";

@NgModule({
  declarations: [APITokensComponent],
  imports: [
    CommonModule,
    TranslateModule,
    GeneralInputModule,
    ConfirmButtonModule
  ],
  exports: [APITokensComponent],
  bootstrap: [APITokensComponent]
})
export class APITokensModule { }
