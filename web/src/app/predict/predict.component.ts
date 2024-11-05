import { CommonModule } from '@angular/common';
import { Component, inject } from '@angular/core';
import { PredictService } from '../../services/predict.service';
import { PredictNavBarComponent } from "./predict-nav-bar/predict-nav-bar.component";

@Component({
  selector: 'app-predict',
  standalone: true,
  imports: [CommonModule, PredictNavBarComponent],
  templateUrl: './predict.component.html',
  styleUrl: './predict.component.scss'
})
export class PredictComponent {
  public predictSvc = inject(PredictService);

}
