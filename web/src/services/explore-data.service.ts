import { HttpParams } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable, tap } from 'rxjs';
import { PlotlyResponse, TableData } from '../models/responses';
import { GridService } from '@shared/grid/grid.service';
import { PlotService } from '@shared/plot/plot.service';


@Injectable({
  providedIn: 'root'
})
export class ExploreDataService {
  private gridService = inject(GridService);
  private plotService = inject(PlotService);

  plotDemographics$(subCategory: string, gender: string = '', with_controls: boolean = false): Observable<PlotlyResponse> {
    const params = new HttpParams().set('display', 'plot').set('sub_category', subCategory).set('gender', gender).set('with_controls', with_controls);
    return this.plotService.getPlot('demographics', params);
  }

  getDemographicsTableData$(subCategory: string, gender: string = '', with_controls: boolean = false): Observable<TableData> {
    const params = new HttpParams().set('display', 'table').set('sub_category', subCategory).set('gender', gender).set('with_controls', with_controls);
    return this.gridService.getTable('demographics', params);
  }

  plotMentalHealth$(subCategory: string, gender: string = '', with_controls: boolean = false): Observable<PlotlyResponse> {
    const params = new HttpParams().set('display', 'plot').set('sub_category', subCategory).set('gender', gender).set('with_controls', with_controls);
    return this.plotService.getPlot('mental-health', params);
  }

  getMentalHealthTableData$(subCategory: string, gender: string = '', with_controls: boolean = false): Observable<TableData> {
    const params = new HttpParams().set('display', 'table').set('sub_category', subCategory).set('gender', gender).set('with_controls', with_controls);
    return this.gridService.getTable('mental-health', params);
  }
}
