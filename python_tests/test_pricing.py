import math
import pytest
import rust_option_engine as roe


def test_call_price():
    price = roe.call_price(
        100.0,
        100.0,
        0.05,
        0.2,
        1.0,
    )

    assert abs(price - 10.450583572185565) < 1e-10

def test_call_price_at_maturity():
    price = roe.call_price(
        100.0,
        100.0,
        0.05,
        0.2,
        0.0,

    )

    assert price == 0.0

def test_put_price():
    price = roe.put_price(
        100.0,
        100.0,
        0.05,
        0.2,
        1.0,
    )

    assert abs(price - 5.573526022256971) < 1e-10


def test_call_delta():
    delta = roe.delta_call(
        100.0,
        100.0,
        0.05,
        0.2,
        1.0,
    )

    assert abs(delta - 0.6368306511756191) < 1e-10


def test_put_call_parity():
    call = roe.call_price(100.0, 100.0, 0.05, 0.2, 1.0)
    put = roe.put_price(100.0, 100.0, 0.05, 0.2, 1.0)

    expected = 100.0 - 100.0 * math.exp(-0.05)

    assert abs((call - put) - expected) < 1e-10


def test_call_delta_bounds():
    delta = roe.delta_call(
        100.0,
        100.0,
        0.05,
        0.2,
        1.0,
    )

    assert 0.0 <= delta <= 1.0, f"delta out of bounds: {delta}"

def test_call_price_rejects_negative_volatility():
    with pytest.raises(ValueError):
        roe.call_price(
            100.0,
            100.0,
            0.05,
            -0.2,
            1.0,

        )

def test_call_price_rejects_zero_volatility():
    with pytest.raises(ValueError):
        roe.call_price(
            100.0,
            100.0,
            0.05,
            0.0,
            1.0,
        )

def test_call_price_rejects_negative_maturity():
    with pytest.raises(ValueError):
        roe.call_price(
            100.0,
            100.0,
            0.05,
            0.2,
            -1.0,
        )
def test_call_price_rejects_zero_spot():
    with pytest.raises(ValueError):
        roe.call_price(
            0.0,
            100.0,
            0.05,
            0.2,
            1.0,
        )

def test_call_price_rejects_negative_spot():
    with pytest.raises(ValueError):
        roe.call_price(
            -100.0,
            100.0,
            0.05,
            0.2,
            1.0,
        )

def test_call_price_rejects_zero_strike():
    with pytest.raises(ValueError):
        roe.call_price(
            100.0,
            0.0,
            0.05,
            0.2,
            1.0,
        )

def test_call_price_rejects_negative_strike():
    with pytest.raises(ValueError):
        roe.call_price(
            100.0,
            -100.0,
            0.05,
            0.2,
            1.0,
        )

def test_put_price_rejects_negative_volatility():
    with pytest.raises(ValueError):
        roe.put_price(100.0, 100.0, 0.05, -0.2, 1.0)


def test_put_price_rejects_negative_maturity():
    with pytest.raises(ValueError):
        roe.put_price(100.0, 100.0, 0.05, 0.2, -1.0)


def test_put_price_rejects_zero_spot():
    with pytest.raises(ValueError):
        roe.put_price(0.0, 100.0, 0.05, 0.2, 1.0)


def test_put_price_rejects_zero_strike():
    with pytest.raises(ValueError):
        roe.put_price(100.0, 0.0, 0.05, 0.2, 1.0)


def test_delta_call_rejects_negative_volatility():
    with pytest.raises(ValueError):
        roe.delta_call(100.0, 100.0, 0.05, -0.2, 1.0)


def test_delta_call_rejects_negative_maturity():
    with pytest.raises(ValueError):
        roe.delta_call(100.0, 100.0, 0.05, 0.2, -1.0)


def test_delta_call_rejects_zero_spot():
    with pytest.raises(ValueError):
        roe.delta_call(0.0, 100.0, 0.05, 0.2, 1.0)


def test_delta_call_rejects_zero_strike():
    with pytest.raises(ValueError):
        roe.delta_call(100.0, 0.0, 0.05, 0.2, 1.0)
