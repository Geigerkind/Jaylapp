import { Component, Output, EventEmitter, Input } from "@angular/core";
import { CookieOption } from "../material/cookie_option";

@Component({
  selector: "CookieOptions",
  templateUrl: "./cookie_options.html",
  styleUrls: ["./cookie_options.scss"]
})
export class CookieOptions {
  @Output() show_front: EventEmitter<boolean> = new EventEmitter();

  @Input() cookies_third_party: Array<CookieOption>;
  @Input() cookies_other: Array<CookieOption>;
  @Input() cookies_necessary: Array<CookieOption>;

  emit_show_front(): void {
    this.show_front.emit(true);
  }

  reject_all(): void {
    this.cookies_other.forEach(cookie => { if (cookie.enabled) { cookie.toggle(); } } );
    this.cookies_third_party.forEach(cookie => { if (cookie.enabled) { cookie.toggle(); } } );
  }
}
