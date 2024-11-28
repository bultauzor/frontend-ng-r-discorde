import {ComponentFixture, TestBed} from '@angular/core/testing';

import {MessageComponent} from './message.component';
import {Message} from '../../models/message';
import {input} from '@angular/core';

describe('MessageComponent', () => {
  let component: MessageComponent;
  let fixture: ComponentFixture<MessageComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MessageComponent]
    })
      .compileComponents();

    fixture = TestBed.createComponent(MessageComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should be correct', async () => {
    const msg = new Message(Date.now() - 3600000, "michel", "foo")
    // @ts-ignore
    component.timestamp = input(await msg.timestamp$.toPromise())
    fixture.detectChanges();
    // check if '1 minute ago'
  })
});
