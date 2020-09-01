def test_basic_usage_1(kosem):
    with kosem.phase(
        kosem.Button('Click me to continue').named('this'),
        kosem.Button('No! click me instead!').named('that'),
    ) as phase:
        phase.wait_for_button()

def test_basic_usage_2(kosem):
    with kosem.phase(kosem.Button('Click me to continue')) as phase:
        phase.wait_for_button()
