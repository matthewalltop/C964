import { CommonModule } from '@angular/common';
import { Component, CUSTOM_ELEMENTS_SCHEMA, inject } from '@angular/core';
import { PredictService } from '@services/predict.service';
import { PredictNavBarComponent } from "./predict-nav-bar/predict-nav-bar.component";
import { PredictRequest } from '@models/requests';
import { PlotlyModule } from 'angular-plotly.js';
import { BehaviorSubject, finalize, tap } from 'rxjs';
import { ConfusionMatrix, MLResponse } from '@models/responses';
import { AsWordsPipe } from '@shared/as-words.pipe';
import { indicate } from '@shared/nils-operators/indicate';

@Component({
  selector: 'app-predict',
  standalone: true,
  imports: [CommonModule, PredictNavBarComponent, PlotlyModule, AsWordsPipe],
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
  providers: [PredictService],
  templateUrl: './predict.component.html',
  styleUrl: './predict.component.scss'
})
export class PredictComponent {
  public predictSvc = inject(PredictService);

  loading$: BehaviorSubject<boolean> = new BehaviorSubject<boolean>(false);

  predictRequest$: BehaviorSubject<PredictRequest | null> = new BehaviorSubject<PredictRequest | null>(null);
  predictResponse$: BehaviorSubject<MLResponse | null> = new BehaviorSubject<MLResponse | null>(null);
  cf_matrix$: BehaviorSubject<ConfusionMatrix | null> = new BehaviorSubject<ConfusionMatrix | null>(null);

  submit(request: PredictRequest) {
    this.predictSvc.make_prediction(request)
      .pipe(
        tap(() => this.predictRequest$.next(request)),
        indicate(this.loading$),
        finalize(() => this.loading$.next(false))
      )
      .subscribe((res) => {
        this.predictResponse$.next(res);

        let cf = this.create_cf_matrix(res.cf_matrix);
        this.cf_matrix$.next(cf);
    });
  }

  create_cf_matrix(res: string): ConfusionMatrix {
    let cf = res.split('\n');

    // The first line in the response is always empty.
    let headerRow: string[] = [],
      rows: string[][] = [];

    for (let i = 1; i < cf.length; i++) {
      let row = cf[i].split('|').map((x) => x.replace(',', '').trim());
      if (i === 1) {
        headerRow = row;
      } else {
        rows.push(row);
      }
    }

    return {
      headerRow,
      rows
    }
  }

}
