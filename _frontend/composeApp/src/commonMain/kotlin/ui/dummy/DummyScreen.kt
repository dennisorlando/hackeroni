package ui.dummy

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import hackathonschenna.composeapp.generated.resources.Res
import hackathonschenna.composeapp.generated.resources.dummy
import io.ktor.client.HttpClient
import org.jetbrains.compose.resources.stringResource
import org.koin.compose.koinInject

@Composable
fun DummyScreen(initialNumber: Int, navigationIcon: @Composable () -> Unit) {
    val httpClient: HttpClient = koinInject()
    val viewModel = viewModel { DummyViewModel(initialNumber, httpClient) }
    val number by viewModel.uiState.collectAsState()

    Scaffold(
        topBar = {
            @OptIn(ExperimentalMaterial3Api::class)
            TopAppBar(
                title = { Text(stringResource(Res.string.dummy)) },
                navigationIcon = navigationIcon
            )
        }
    ) { paddingValues ->
        DummyScreen(
            number = number,
            increase = viewModel::increase,
            decrease = viewModel::decrease,
            random = viewModel::random,
            modifier = Modifier.padding(paddingValues)
        )
    }
}

@Composable
fun DummyScreen(
    number: Int,
    increase: () -> Unit,
    decrease: () -> Unit,
    random: () -> Unit,
    modifier: Modifier = Modifier,
) {
    Column(
        modifier = modifier
    ) {
        Text("Number is $number")

        Button(onClick = increase) {
            Text("Increase")
        }

        Button(onClick = decrease) {
            Text("Decrease")
        }

        Button(onClick = random) {
            Text("Random")
        }
    }
}
