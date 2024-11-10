import { CommonModule } from '@angular/common';
import { Component, CUSTOM_ELEMENTS_SCHEMA, inject } from '@angular/core';
import { PredictService } from '@services/predict.service';
import { PredictNavBarComponent } from "./predict-nav-bar/predict-nav-bar.component";
import { PredictRequest } from '@models/requests';
import { PlotlyModule } from 'angular-plotly.js';

@Component({
  selector: 'app-predict',
  standalone: true,
  imports: [CommonModule, PredictNavBarComponent, PlotlyModule],
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
  providers: [PredictService],
  templateUrl: './predict.component.html',
  styleUrl: './predict.component.scss'
})
export class PredictComponent {
  public predictSvc = inject(PredictService);


  submit(event: PredictRequest) {
    console.log(event);
  }
}
