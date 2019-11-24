import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { ConfirmButtonComponent } from "./confirm_button";

@NgModule({
  declarations: [ConfirmButtonComponent],
  imports: [
    CommonModule,
    TranslateModule
  ],
  exports: [ConfirmButtonComponent]
})
export class ConfirmButtonModule { }
