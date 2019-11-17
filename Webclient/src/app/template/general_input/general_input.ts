import { Component, Input } from "@angular/core";

@Component({
  selector: "GeneralInput",
  templateUrl: "./general_input.html",
  styleUrls: ["./general_input.scss"]
})
export class GeneralInput {
  @Input() type: string;
  @Input() placeholderKey: string;
  @Input() initialValue: string;
  @Input() labelKey: string;
  @Input() required: boolean;
  @Input() minimum_length: number;
  @Input() name: string;
}
