package ui.map

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.lifecycle.viewmodel.compose.viewModel
import com.javiersc.kotlinx.coroutines.run.blocking.runBlocking
import hackathonschenna.composeapp.generated.resources.Res
import hackathonschenna.composeapp.generated.resources.dummy
import io.ktor.client.HttpClient
import io.ktor.client.request.get
import io.ktor.client.statement.bodyAsChannel
import io.ktor.util.cio.toByteArray
import io.ktor.utils.io.ByteReadChannel
import io.ktor.utils.io.cancel
import kotlinx.io.Buffer
import kotlinx.io.RawSource
import org.jetbrains.compose.resources.stringResource
import org.koin.compose.koinInject
import ovh.plrapps.mapcompose.api.addLayer
import ovh.plrapps.mapcompose.api.enableRotation
import ovh.plrapps.mapcompose.core.TileStreamProvider
import ovh.plrapps.mapcompose.ui.MapUI
import ovh.plrapps.mapcompose.ui.state.MapState

@Composable
fun MapScreen(navigationIcon: @Composable () -> Unit) {
    val httpClient: HttpClient = koinInject()
    val viewModel = viewModel { MapViewModel(0, httpClient) }
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
        MapScreen(
            modifier = Modifier.padding(paddingValues)
        )
    }
}

private fun ByteReadChannel.toRawSource(): RawSource {
    val src = this

    return object: RawSource {
        override fun close() {
            src.cancel()
        }

        override fun readAtMostTo(sink: Buffer, byteCount: Long): Long {
            if (byteCount == 0L) return 0L

            require(byteCount >= 0) { "byteCount ($byteCount) < 0" }

            try {
                val ba = runBlocking { src.toByteArray(byteCount.toInt()) }

                for (b in ba) {
                    sink.writeByte(b)
                }

                if (byteCount.toInt() > 0) {
                    println("readAtMostTo, byteCount=${byteCount.toInt()}, actual=${ba.size.toLong()}")
                }
                return ba.size.toLong()
            } catch (e: AssertionError) {
                throw e
            }
        }
    }
}

@Composable
fun MapScreen(
    modifier: Modifier = Modifier,
) {
    val httpClient: HttpClient = koinInject()
    val state = remember { MapState(4, 4096, 4096).apply {
        addLayer({ row, col, zoomLvl ->
            println("Request to $row $col $zoomLvl https://tile.openstreetmap.org/$zoomLvl/$row/$col.png")
            httpClient.get("https://tile.openstreetmap.org/18/148342/101158.png")//"https://tile.openstreetmap.org/$zoomLvl/$row/$col.png")
                .bodyAsChannel()
                .toRawSource()
        })
        enableRotation()
    } }
    MapUI(modifier = modifier.fillMaxSize(), state = state)
}
