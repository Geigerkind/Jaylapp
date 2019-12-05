import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {CommonModule} from "@angular/common";
import {UpdateMailComponent} from "./component/update_mail/update_mail";
import {PasswordInputModule} from "src/app/template/input/password_input/module";
import {ConfirmButtonModule} from "src/app/template/button/confirm_button/module";
import {GeneralInputModule} from "src/app/template/input/general_input/module";
import {BriefNoteModule} from "src/app/template/brief_note/module";
import {UpdateMailRouting} from "./routing";

@NgModule({
    declarations: [UpdateMailComponent],
    imports: [
        CommonModule,
        TranslateModule,
        GeneralInputModule,
        PasswordInputModule,
        ConfirmButtonModule,
        BriefNoteModule,
        UpdateMailRouting
    ],
    exports: [UpdateMailComponent],
    bootstrap: [UpdateMailComponent]
})
export class UpdateMailModule {
}
