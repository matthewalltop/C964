import { TestBed } from '@angular/core/testing';

import { MentalHealthService } from './mental-health.service';

describe('MentalHealthService', () => {
  let service: MentalHealthService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(MentalHealthService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
